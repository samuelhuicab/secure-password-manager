import { useEffect, useRef } from "react";

import { getSettings, touchActivity } from "../services/auth";

const ACTIVITY_EVENTS = ["mousedown", "keydown", "wheel", "touchstart"];

/**
 * Bloquea el vault tras un periodo de inactividad. El tiempo se lee de
 * settings.json (`autoLockSeconds`); `0` desactiva el auto-bloqueo.
 *
 * @param {boolean} active  Solo corre cuando el vault está desbloqueado.
 * @param {() => void} onLock  Callback que bloquea (AuthContext.lock).
 */
export default function useAutoLock(active, onLock) {

    const timer = useRef(null);
    const lastPing = useRef(0);

    useEffect(() => {

        if (!active) return;

        let seconds = 300;
        let cancelled = false;

        function clear() {
            if (timer.current) clearTimeout(timer.current);
        }

        function schedule() {
            clear();
            if (seconds <= 0) return;
            timer.current = setTimeout(() => onLock(), seconds * 1000);
        }

        function onActivity() {
            const now = Date.now();
            schedule();
            // No spammeamos el backend: como mucho un ping cada 15 s.
            if (now - lastPing.current > 15000) {
                lastPing.current = now;
                touchActivity().catch(() => {});
            }
        }

        getSettings()
            .then((s) => {
                if (cancelled) return;
                seconds = s.autoLockSeconds ?? 300;
                schedule();
            })
            .catch(() => schedule());

        ACTIVITY_EVENTS.forEach((e) =>
            window.addEventListener(e, onActivity, { passive: true })
        );

        return () => {
            cancelled = true;
            clear();
            ACTIVITY_EVENTS.forEach((e) => window.removeEventListener(e, onActivity));
        };

    }, [active, onLock]);

}
