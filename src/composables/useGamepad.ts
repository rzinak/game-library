import { onMounted, onUnmounted, toValue, type MaybeRef } from "vue";

export type GamepadAction = "up" | "down" | "left" | "right" | "a" | "b" | "lb" | "rb";

export interface GamepadProfile {
  /** Maps physical button index → named action */
  buttons: Partial<Record<number, GamepadAction>>;
  /** Left stick axis indices (default: x=0, y=1) */
  axes?: { x?: number; y?: number };
  /** Stick dead-zone threshold (default: 0.5) */
  threshold?: number;
}

/** Standard W3C mapping — covers Xbox One/Series and PS4/PS5 in modern browsers */
export const STANDARD_PROFILE: GamepadProfile = {
  buttons: {
    0:  "a",     // Xbox: A    / PS: Cross
    1:  "b",     // Xbox: B    / PS: Circle
    4:  "lb",    // Xbox: LB   / PS: L1
    5:  "rb",    // Xbox: RB   / PS: R1
    12: "up",
    13: "down",
    14: "left",
    15: "right",
  },
  axes: { x: 0, y: 1 },
};

export interface UseGamepadOptions {
  /**
   * When false, actions are suppressed but button state is still tracked
   * so no ghost presses fire when the composable is re-enabled. Default: true
   */
  enabled?: MaybeRef<boolean>;
  /** Time (ms) after first press before repeating. Default: 400 */
  repeatDelay?: number;
  /** Time (ms) between repeated presses while held. Default: 150 */
  repeatInterval?: number;
  /** Button profile to use. Default: STANDARD_PROFILE */
  profile?: GamepadProfile;
}

interface BtnState {
  pressed: boolean;
  pressedAt: number;   // timestamp of initial press (for initial delay)
  lastFiredAt: number; // timestamp of last fire (for repeat interval)
}

export function useGamepad(
  handler: (action: GamepadAction) => void,
  options?: UseGamepadOptions,
): void {
  const repeatDelay = options?.repeatDelay ?? 400;
  const repeatInterval = options?.repeatInterval ?? 150;
  const profile = options?.profile ?? STANDARD_PROFILE;

  const axisX = profile.axes?.x ?? 0;
  const axisY = profile.axes?.y ?? 1;
  const threshold = profile.threshold ?? 0.5;

  const buttonEntries = Object.entries(profile.buttons) as [string, GamepadAction][];

  const state = new Map<string, BtnState>();
  let rafId = 0;
  let prevEnabled = true;

  function snapshotAll(pads: readonly (Gamepad | null)[]) {
    for (const pad of pads) {
      if (!pad) continue;
      for (const [idxStr, action] of buttonEntries) {
        const btnIdx = Number(idxStr);
        const id = `${pad.index}-${action}`;
        if (isPhysical(pad, btnIdx, action)) {
          // Use Infinity for pressedAt so repeat-fire can never trigger:
          // the button must be fully released and freshly pressed to produce an action.
          state.set(id, { pressed: true, pressedAt: Infinity, lastFiredAt: Infinity });
        }
      }
    }
  }

  function isPhysical(pad: Gamepad, btnIdx: number, action: GamepadAction): boolean {
    const button = pad.buttons[btnIdx];
    if (!button) return false;
    const stickX = pad.axes[axisX] ?? 0;
    const stickY = pad.axes[axisY] ?? 0;
    return (
      button.pressed ||
      (action === "right" && stickX >  threshold) ||
      (action === "left"  && stickX < -threshold) ||
      (action === "down"  && stickY >  threshold) ||
      (action === "up"    && stickY < -threshold)
    );
  }

  function poll() {
    const pads = navigator.getGamepads();
    const now = performance.now();
    const enabled = options?.enabled === undefined ? true : toValue(options.enabled);

    // On false → true transition, snapshot current button state so any button
    // held during the disabled period (or on a reconnected pad) doesn't ghost-fire.
    if (enabled && !prevEnabled) {
      snapshotAll(pads);
    }
    prevEnabled = enabled;

    for (const pad of pads) {
      if (!pad) continue;

      for (const [idxStr, action] of buttonEntries) {
        const btnIdx = Number(idxStr);
        const id = `${pad.index}-${action}`;
        const s = state.get(id) ?? { pressed: false, pressedAt: 0, lastFiredAt: 0 };
        const physical = isPhysical(pad, btnIdx, action);

        if (physical) {
          if (!enabled) {
            // Mirror state but suppress handler so no ghost fires when re-enabled.
            // lastFiredAt = now ensures that on re-enable, now - lastFiredAt ≈ 16ms < repeatInterval.
            state.set(id, {
              pressed: true,
              pressedAt: s.pressedAt || now,
              lastFiredAt: now,
            });
            continue;
          }

          const shouldFire =
            !s.pressed ||
            (now - s.pressedAt > repeatDelay && now - s.lastFiredAt > repeatInterval);

          if (shouldFire) {
            handler(action);
            if (!s.pressed) {
              // First press
              state.set(id, { pressed: true, pressedAt: now, lastFiredAt: now });
            } else {
              // Repeat fire — preserve pressedAt
              state.set(id, { ...s, lastFiredAt: now });
            }
          }
        } else if (s.pressed) {
          state.set(id, { pressed: false, pressedAt: 0, lastFiredAt: 0 });
        }
      }
    }

    rafId = requestAnimationFrame(poll);
  }

  onMounted(() => {
    // Pre-populate state with currently-held buttons to prevent ghost presses
    // (e.g. the "A" press that opened a modal is still physically held).
    const now = performance.now();
    for (const pad of navigator.getGamepads()) {
      if (!pad) continue;
      for (const [idxStr, action] of buttonEntries) {
        const btnIdx = Number(idxStr);
        if (isPhysical(pad, btnIdx, action)) {
          const id = `${pad.index}-${action}`;
          state.set(id, { pressed: true, pressedAt: now, lastFiredAt: now });
        }
      }
    }

    rafId = requestAnimationFrame(poll);
  });

  onUnmounted(() => {
    cancelAnimationFrame(rafId);
  });
}
