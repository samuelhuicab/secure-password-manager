import {
    createContext,
    useCallback,
    useContext,
    useEffect,
    useState,
} from "react";

import {
    vaultStatus,
    createMasterPassword,
    unlockVault,
    unlockWithKeychain,
    lockVault,
} from "../services/auth";

import { setLockHandler } from "../services/vault";

const AuthContext = createContext(null);

// status: "loading" | "uninitialized" | "locked" | "unlocked"

export function AuthProvider({ children }) {

    const [status, setStatus] = useState("loading");

    const [hasKeychain, setHasKeychain] = useState(false);

    const [rememberDevice, setRememberDevice] = useState(false);

    async function refresh() {

        const s = await vaultStatus();

        setHasKeychain(s.hasKeychain);

        setRememberDevice(s.rememberDevice);

        if (!s.initialized) {
            setStatus("uninitialized");
        } else if (s.locked) {
            setStatus("locked");
        } else {
            setStatus("unlocked");
        }

        return s;

    }

    async function createMaster(password) {
        await createMasterPassword(password);
        await refresh();
    }

    async function unlock(password, remember) {
        await unlockVault(password, remember);
        await refresh();
    }

    async function unlockKeychain() {
        await unlockWithKeychain();
        await refresh();
    }

    async function lock() {
        try {
            await lockVault();
        } finally {
            setStatus("locked");
        }
    }

    // Si cualquier comando devuelve VAULT_LOCKED (sesión caducada), bajamos a
    // la pantalla de bloqueo.
    useEffect(() => {
        setLockHandler(() => setStatus("locked"));
        return () => setLockHandler(null);
    }, []);

    useEffect(() => {
        refresh().catch(() => setStatus("uninitialized"));
    }, []);

    // Ctrl+L / Cmd+L: bloqueo inmediato.
    useEffect(() => {
        function onKey(e) {
            if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "l") {
                e.preventDefault();
                lock();
            }
        }
        window.addEventListener("keydown", onKey);
        return () => window.removeEventListener("keydown", onKey);
    }, []);

    const value = {
        status,
        hasKeychain,
        rememberDevice,
        refresh,
        createMaster,
        unlock,
        unlockKeychain,
        lock,
    };

    return (
        <AuthContext.Provider value={value}>
            {children}
        </AuthContext.Provider>
    );

}

export function useAuth() {
    return useContext(AuthContext);
}
