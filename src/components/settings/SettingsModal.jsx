import { useEffect, useState } from "react";

import { save as saveDialog, open as openDialog } from "@tauri-apps/plugin-dialog";

import Modal from "../ui/Modal";
import Button from "../ui/Button";

import { useAuth } from "../../contexts/AuthContext";
import {
    getSettings,
    updateSettings,
    changeMasterPassword,
    forgetKeychain,
} from "../../services/auth";
import { exportBackup, importBackup } from "../../services/dev";

const LOCK_OPTIONS = [
    { v: 0, label: "Nunca" },
    { v: 60, label: "1 minuto" },
    { v: 300, label: "5 minutos" },
    { v: 600, label: "10 minutos" },
    { v: 1800, label: "30 minutos" },
];

const pwInput =
    "w-full h-10 bg-zinc-900 border border-zinc-800 rounded-lg px-3 text-sm text-white outline-none focus:border-blue-500";

export default function SettingsModal({ onClose }) {

    const { hasKeychain, refresh, lock } = useAuth();

    const [settings, setSettings] = useState(null);
    const [msg, setMsg] = useState("");
    const [busy, setBusy] = useState(false);

    const [cur, setCur] = useState("");
    const [next, setNext] = useState("");
    const [next2, setNext2] = useState("");

    const [backupMode, setBackupMode] = useState(null); // "export" | "import" | null
    const [backupPw, setBackupPw] = useState("");
    const [importAck, setImportAck] = useState(false);

    useEffect(() => {
        getSettings().then(setSettings).catch(() =>
            setSettings({ autoLockSeconds: 300, rememberDevice: false, clipboardClearSeconds: 20 })
        );
    }, []);

    if (!settings) return null;

    async function persist(patch) {
        const updated = { ...settings, ...patch };
        setSettings(updated);
        try {
            await updateSettings(updated);
        } catch (err) {
            setMsg(String(err));
        }
    }

    async function handleChangePassword() {
        setMsg("");
        if (next.length < 8) return setMsg("La nueva contraseña necesita 8+ caracteres.");
        if (next !== next2) return setMsg("Las contraseñas nuevas no coinciden.");
        setBusy(true);
        try {
            await changeMasterPassword(cur, next);
            setCur(""); setNext(""); setNext2("");
            setMsg("Contraseña maestra actualizada.");
        } catch (err) {
            setMsg(String(err));
        } finally {
            setBusy(false);
        }
    }

    async function handleForget() {
        try {
            await forgetKeychain();
            await refresh();
            setMsg("Se olvidó la clave de este equipo.");
        } catch (err) {
            setMsg(String(err));
        }
    }

    function openBackup(mode) {
        setBackupMode(mode);
        setBackupPw("");
        setImportAck(false);
        setMsg("");
    }

    async function runExport() {
        if (backupPw.length < 8) return setMsg("La contraseña del backup necesita 8+ caracteres.");
        setBusy(true);
        try {
            const path = await saveDialog({
                defaultPath: "passcontroller-backup.vault",
                filters: [{ name: "Vault", extensions: ["vault"] }],
            });
            if (!path) return setBusy(false);
            await exportBackup(path, backupPw);
            setMsg("Backup exportado.");
            setBackupMode(null);
        } catch (err) {
            setMsg(String(err));
        } finally {
            setBusy(false);
        }
    }

    async function runImport() {
        if (!importAck) return setMsg("Confirmá que entendés que se reemplaza todo el vault.");
        if (!backupPw) return setMsg("Ingresá la contraseña del backup.");
        setBusy(true);
        try {
            const path = await openDialog({
                multiple: false,
                filters: [{ name: "Vault", extensions: ["vault"] }],
            });
            if (!path) return setBusy(false);
            await importBackup(path, backupPw);
            setMsg("Backup importado. Se bloqueará para recargar.");
            setTimeout(() => lock(), 900);
        } catch (err) {
            setMsg(String(err));
        } finally {
            setBusy(false);
        }
    }

    return (

        <Modal title="Ajustes" onClose={onClose}>

            <div className="space-y-6">

                <div>
                    <label className="block text-sm text-zinc-400 mb-2">
                        Bloqueo automático por inactividad
                    </label>
                    <select
                        value={settings.autoLockSeconds}
                        onChange={(e) => persist({ autoLockSeconds: Number(e.target.value) })}
                        className="w-full h-11 bg-zinc-900 border border-zinc-800 rounded-lg px-4 text-sm text-white outline-none focus:border-blue-500"
                    >
                        {LOCK_OPTIONS.map((o) => (
                            <option key={o.v} value={o.v}>{o.label}</option>
                        ))}
                    </select>
                </div>

                <div>
                    <label className="block text-sm text-zinc-400 mb-2">
                        Limpiar portapapeles tras copiar un secreto
                    </label>
                    <select
                        value={settings.clipboardClearSeconds}
                        onChange={(e) => persist({ clipboardClearSeconds: Number(e.target.value) })}
                        className="w-full h-11 bg-zinc-900 border border-zinc-800 rounded-lg px-4 text-sm text-white outline-none focus:border-blue-500"
                    >
                        <option value={0}>Nunca</option>
                        <option value={10}>10 segundos</option>
                        <option value={20}>20 segundos</option>
                        <option value={45}>45 segundos</option>
                        <option value={90}>90 segundos</option>
                    </select>
                </div>

                {hasKeychain && (
                    <div className="flex items-center justify-between">
                        <span className="text-sm text-zinc-400">
                            Clave recordada en el llavero del sistema
                        </span>
                        <Button variant="secondary" onClick={handleForget}>
                            Olvidar
                        </Button>
                    </div>
                )}

                <div className="border-t border-zinc-800 pt-5">
                    <h3 className="text-sm font-semibold text-white mb-3">
                        Cambiar contraseña maestra
                    </h3>
                    <div className="space-y-2">
                        <input type="password" placeholder="Actual" value={cur}
                            onChange={(e) => setCur(e.target.value)} className={pwInput} />
                        <input type="password" placeholder="Nueva" value={next}
                            onChange={(e) => setNext(e.target.value)} className={pwInput} />
                        <input type="password" placeholder="Repetir nueva" value={next2}
                            onChange={(e) => setNext2(e.target.value)} className={pwInput} />
                        <Button variant="secondary" onClick={handleChangePassword} disabled={busy}>
                            Actualizar contraseña
                        </Button>
                    </div>
                </div>

                <div className="border-t border-zinc-800 pt-5">
                    <h3 className="text-sm font-semibold text-white mb-3">
                        Backup cifrado
                    </h3>

                    {backupMode === null && (
                        <div className="flex gap-3">
                            <Button variant="secondary" onClick={() => openBackup("export")}>
                                Exportar
                            </Button>
                            <Button variant="secondary" onClick={() => openBackup("import")}>
                                Importar
                            </Button>
                        </div>
                    )}

                    {backupMode === "export" && (
                        <div className="space-y-2">
                            <input
                                type="password"
                                autoFocus
                                placeholder="Contraseña para el backup (8+)"
                                value={backupPw}
                                onChange={(e) => setBackupPw(e.target.value)}
                                className={pwInput}
                            />
                            <div className="flex gap-2">
                                <Button variant="primary" onClick={runExport} disabled={busy}>
                                    Elegir archivo y exportar
                                </Button>
                                <Button variant="ghost" onClick={() => setBackupMode(null)}>
                                    Cancelar
                                </Button>
                            </div>
                        </div>
                    )}

                    {backupMode === "import" && (
                        <div className="space-y-2">
                            <label className="flex items-start gap-2 text-xs text-amber-400 select-none cursor-pointer">
                                <input
                                    type="checkbox"
                                    checked={importAck}
                                    onChange={(e) => setImportAck(e.target.checked)}
                                    className="accent-blue-600 mt-0.5"
                                />
                                Entiendo que importar reemplaza TODO el contenido actual del vault.
                            </label>
                            <input
                                type="password"
                                placeholder="Contraseña del backup"
                                value={backupPw}
                                onChange={(e) => setBackupPw(e.target.value)}
                                className={pwInput}
                            />
                            <div className="flex gap-2">
                                <Button variant="danger" onClick={runImport} disabled={busy}>
                                    Elegir archivo e importar
                                </Button>
                                <Button variant="ghost" onClick={() => setBackupMode(null)}>
                                    Cancelar
                                </Button>
                            </div>
                        </div>
                    )}
                </div>

                {msg && (
                    <div className="text-sm text-zinc-300 bg-zinc-900 border border-zinc-800 rounded-lg px-3 py-2">
                        {msg}
                    </div>
                )}

            </div>

        </Modal>

    );

}
