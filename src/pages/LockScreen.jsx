import { useState } from "react";

import { ShieldCheck, Lock, KeyRound, Loader2 } from "lucide-react";

import { useAuth } from "../contexts/AuthContext";

function strength(pw) {
    let score = 0;
    if (pw.length >= 8) score++;
    if (pw.length >= 12) score++;
    if (/[A-Z]/.test(pw) && /[a-z]/.test(pw)) score++;
    if (/\d/.test(pw)) score++;
    if (/[^A-Za-z0-9]/.test(pw)) score++;
    return Math.min(score, 4);
}

const STRENGTH_LABEL = ["Muy débil", "Débil", "Aceptable", "Buena", "Fuerte"];
const STRENGTH_COLOR = [
    "bg-red-500",
    "bg-orange-500",
    "bg-yellow-500",
    "bg-lime-500",
    "bg-emerald-500",
];

export default function LockScreen() {

    const { status, hasKeychain, createMaster, unlock, unlockKeychain } = useAuth();

    const creating = status === "uninitialized";

    const [password, setPassword] = useState("");
    const [confirm, setConfirm] = useState("");
    const [remember, setRemember] = useState(false);
    const [busy, setBusy] = useState(false);
    const [error, setError] = useState("");

    const score = strength(password);

    async function handleSubmit(e) {
        e.preventDefault();
        setError("");

        if (creating) {
            if (password.length < 8) {
                setError("Usá al menos 8 caracteres.");
                return;
            }
            if (password !== confirm) {
                setError("Las contraseñas no coinciden.");
                return;
            }
        }

        setBusy(true);
        try {
            if (creating) {
                await createMaster(password);
            } else {
                await unlock(password, remember);
            }
        } catch (err) {
            setError(String(err));
            setBusy(false);
        }
    }

    async function handleKeychain() {
        setError("");
        setBusy(true);
        try {
            await unlockKeychain();
        } catch (err) {
            setError(String(err));
            setBusy(false);
        }
    }

    const inputClass =
        "w-full h-11 bg-zinc-900 border border-zinc-800 rounded-lg px-4 text-sm text-white outline-none transition-colors focus:border-blue-500";

    return (

        <div className="h-screen flex items-center justify-center bg-zinc-950 p-4">

            <div className="w-full max-w-sm">

                <div className="flex flex-col items-center mb-8">
                    <div className="w-12 h-12 rounded-xl bg-zinc-900 border border-zinc-800 flex items-center justify-center text-blue-500">
                        <ShieldCheck size={22} />
                    </div>
                    <h1 className="mt-4 text-lg font-semibold text-white">
                        {creating ? "Creá tu contraseña maestra" : "Vault bloqueado"}
                    </h1>
                    <p className="mt-1 text-sm text-zinc-500 text-center">
                        {creating
                            ? "Cifra todo el vault. No se puede recuperar si la perdés."
                            : "Ingresá tu contraseña maestra para continuar."}
                    </p>
                </div>

                <form onSubmit={handleSubmit} className="space-y-4">

                    <div>
                        <label className="block text-sm text-zinc-400 mb-2">
                            Contraseña maestra
                        </label>
                        <input
                            type="password"
                            value={password}
                            autoFocus
                            onChange={(e) => setPassword(e.target.value)}
                            className={inputClass}
                        />
                        {creating && password && (
                            <div className="mt-2">
                                <div className="flex gap-1">
                                    {[0, 1, 2, 3].map((i) => (
                                        <div
                                            key={i}
                                            className={`h-1 flex-1 rounded-full transition-colors ${
                                                i <= score ? STRENGTH_COLOR[score] : "bg-zinc-800"
                                            }`}
                                        />
                                    ))}
                                </div>
                                <span className="mt-1.5 block text-xs text-zinc-500">
                                    {STRENGTH_LABEL[score]}
                                </span>
                            </div>
                        )}
                    </div>

                    {creating && (
                        <div>
                            <label className="block text-sm text-zinc-400 mb-2">
                                Repetí la contraseña
                            </label>
                            <input
                                type="password"
                                value={confirm}
                                onChange={(e) => setConfirm(e.target.value)}
                                className={inputClass}
                            />
                        </div>
                    )}

                    {!creating && (
                        <label className="flex items-center gap-2 text-sm text-zinc-400 select-none cursor-pointer">
                            <input
                                type="checkbox"
                                checked={remember}
                                onChange={(e) => setRemember(e.target.checked)}
                                className="accent-blue-600"
                            />
                            Recordar en este equipo
                        </label>
                    )}

                    {error && (
                        <div className="text-sm text-red-400 bg-red-950/40 border border-red-900/50 rounded-lg px-3 py-2">
                            {error}
                        </div>
                    )}

                    <button
                        type="submit"
                        disabled={busy}
                        className="w-full h-11 rounded-lg bg-blue-600 hover:bg-blue-500 disabled:opacity-40 disabled:cursor-not-allowed text-white text-sm font-medium inline-flex items-center justify-center gap-2 transition-colors"
                    >
                        {busy ? <Loader2 size={16} className="animate-spin" /> : <Lock size={16} />}
                        {creating ? "Crear y desbloquear" : "Desbloquear"}
                    </button>

                    {!creating && hasKeychain && (
                        <button
                            type="button"
                            onClick={handleKeychain}
                            disabled={busy}
                            className="w-full h-11 rounded-lg bg-zinc-900 border border-zinc-800 hover:border-zinc-700 disabled:opacity-40 text-zinc-200 text-sm font-medium inline-flex items-center justify-center gap-2 transition-colors"
                        >
                            <KeyRound size={16} />
                            Usar el llavero del sistema
                        </button>
                    )}

                </form>

            </div>

        </div>

    );

}
