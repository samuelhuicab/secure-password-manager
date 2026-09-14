import { useEffect, useState } from "react";

import { Save, Trash2, Terminal, FileText, Plus, FileDown, FileUp } from "lucide-react";

import { save as saveDialog, open as openDialog } from "@tauri-apps/plugin-dialog";

import useVault from "../../hooks/useVault";

import FieldEditor from "./FieldEditor";
import CopyMenu from "../dev/CopyMenu";

import { getTypeIcon, getTypeLabel } from "../../constants/itemTypes";

import {
    updateItemTitle,
    updateItemNotes,
    updateItemField,
    deleteItem,
    openSsh,
    getItemsByNode,
} from "../../services/vault";

import {
    addItemField,
    removeItemField,
    exportEnv,
    importEnv,
} from "../../services/dev";

export default function ItemEditor() {

    const {
        selectedItem,
        setSelectedItem,
        selectedNode,
        loadItems,
    } = useVault();

    const [draft, setDraft] = useState(null);
    const [saving, setSaving] = useState(false);
    const [dirty, setDirty] = useState(false);
    const [connecting, setConnecting] = useState(false);
    const [rev, setRev] = useState(0);

    const [addingField, setAddingField] = useState(false);
    const [newFieldLabel, setNewFieldLabel] = useState("");
    const [newFieldHidden, setNewFieldHidden] = useState(false);
    const [fieldBusy, setFieldBusy] = useState(false);
    const [fieldError, setFieldError] = useState("");

    // Sincroniza el borrador cuando cambia el item o su estructura (rev).
    useEffect(() => {

        if (!selectedItem) {
            setDraft(null);
            setDirty(false);
            return;
        }

        setDraft({
            title: selectedItem.title,
            notes: selectedItem.notes,
            fields: selectedItem.fields.map((f) => ({ ...f })),
        });
        setDirty(false);

    }, [selectedItem?.id, rev]);

    async function refreshItem() {
        const nodeId = selectedItem.node_id;
        const items = await getItemsByNode(nodeId);
        if (selectedNode?.id === nodeId) await loadItems(nodeId);
        const fresh = items.find((i) => i.id === selectedItem.id);
        if (fresh) {
            setSelectedItem(fresh);
            setRev((r) => r + 1);
        }
    }

    if (!selectedItem || !draft) {
        return (
            <section className="flex-1 bg-zinc-950 flex items-center justify-center">
                <div className="text-center max-w-xs">
                    <div className="w-14 h-14 rounded-xl bg-zinc-900 border border-zinc-800 flex items-center justify-center mx-auto text-zinc-600">
                        <FileText size={24} />
                    </div>
                    <h2 className="mt-4 text-lg font-semibold text-white">
                        Ningún elemento seleccionado
                    </h2>
                    <p className="mt-2 text-sm text-zinc-500">
                        Elegí un elemento de la lista para ver y editar su información.
                    </p>
                </div>
            </section>
        );
    }

    function getFieldValue(key) {
        return draft.fields.find((f) => f.key === key)?.value || "";
    }

    function changeTitle(value) {
        setDraft({ ...draft, title: value });
        setDirty(true);
    }

    function changeNotes(value) {
        setDraft({ ...draft, notes: value });
        setDirty(true);
    }

    function changeField(fieldId, value) {
        setDraft({
            ...draft,
            fields: draft.fields.map((f) =>
                f.id === fieldId ? { ...f, value } : f
            ),
        });
        setDirty(true);
    }

    function openAddField() {
        setNewFieldLabel("");
        setNewFieldHidden(false);
        setFieldError("");
        setAddingField(true);
    }

    async function handleAddField(e) {
        e?.preventDefault();
        const label = newFieldLabel.trim();
        if (!label) {
            setFieldError("Escribí un nombre para el campo.");
            return;
        }
        setFieldBusy(true);
        setFieldError("");
        try {
            await addItemField(selectedItem.id, label, newFieldHidden);
            await refreshItem();
            setAddingField(false);
            setNewFieldLabel("");
            setNewFieldHidden(false);
        } catch (err) {
            setFieldError(String(err));
        } finally {
            setFieldBusy(false);
        }
    }

    async function handleRemoveField(field) {
        try {
            await removeItemField(selectedItem.id, field.id);
            await refreshItem();
        } catch (err) {
            alert(String(err));
        }
    }

    async function handleExportEnv() {
        const path = await saveDialog({
            defaultPath: `${selectedItem.title || "secrets"}.env`,
            filters: [{ name: "Env", extensions: ["env"] }],
        });
        if (!path) return;
        await exportEnv(selectedItem.id, path);
    }

    async function handleImportEnv() {
        const path = await openDialog({
            multiple: false,
            filters: [{ name: "Env", extensions: ["env"] }],
        });
        if (!path) return;
        await importEnv(selectedItem.node_id, `${selectedItem.title} (import)`, { path });
        await loadItems(selectedNode.id);
    }

    async function handleSave() {
        setSaving(true);
        try {
            const titleChanged = draft.title !== selectedItem.title;
            const notesChanged = draft.notes !== selectedItem.notes;
            const changedFields = draft.fields.filter((df) => {
                const original = selectedItem.fields.find((f) => f.id === df.id);
                return original && original.value !== df.value;
            });

            if (titleChanged) await updateItemTitle(selectedItem.id, draft.title);
            if (notesChanged) await updateItemNotes(selectedItem.id, draft.notes);
            for (const field of changedFields) {
                await updateItemField(selectedItem.id, field.key, field.value);
            }

            setSelectedItem({
                ...selectedItem,
                title: draft.title,
                notes: draft.notes,
                fields: draft.fields,
            });
            setDirty(false);

            if (selectedNode) await loadItems(selectedNode.id);
        } finally {
            setSaving(false);
        }
    }

    async function handleDelete() {
        if (!confirm(`¿Eliminar "${selectedItem.title}" de forma permanente?`)) return;
        await deleteItem(selectedItem.id);
        setSelectedItem(null);
        if (selectedNode) await loadItems(selectedNode.id);
    }

    async function handleConnectSsh() {
        const host = getFieldValue("host");
        const username = getFieldValue("username");
        if (!host || !username) {
            alert("Ingresa 'Host' y 'Usuario' antes de conectar.");
            return;
        }
        if (dirty && !confirm("Tenés cambios sin guardar. Se conecta con lo que hay en pantalla. ¿Continuar?")) {
            return;
        }
        setConnecting(true);
        try {
            await openSsh(host, username);
        } catch (err) {
            alert(`No se pudo abrir la conexión SSH: ${err}`);
        } finally {
            setConnecting(false);
        }
    }

    const isServer = selectedItem.item_type === "Server";
    const isEnv = selectedItem.item_type === "Env";
    const TypeIcon = getTypeIcon(selectedItem.item_type);

    return (

        <section className="flex-1 overflow-auto bg-zinc-950">

            <div className="border-b border-zinc-800 px-8 py-5">

                <div className="flex items-start gap-3">
                    <div className="w-10 h-10 rounded-lg bg-zinc-900 border border-zinc-800 flex items-center justify-center shrink-0 mt-0.5 text-zinc-400">
                        <TypeIcon size={18} />
                    </div>
                    <div className="min-w-0 flex-1">
                        <input
                            value={draft.title}
                            onChange={(e) => changeTitle(e.target.value)}
                            placeholder="Sin título"
                            className="block w-full bg-transparent text-xl font-semibold text-white outline-none placeholder:text-zinc-600 truncate"
                        />
                        <div className="flex items-center gap-2 mt-1 text-xs text-zinc-500">
                            <span>{getTypeLabel(selectedItem.item_type)}</span>
                            {dirty && (
                                <>
                                    <span className="text-zinc-700">•</span>
                                    <span className="text-amber-500 flex items-center gap-1">
                                        <span className="w-1.5 h-1.5 rounded-full bg-amber-500" />
                                        Sin guardar
                                    </span>
                                </>
                            )}
                        </div>
                    </div>
                </div>

                <div className="flex flex-wrap items-center gap-2 mt-4">

                    {isEnv && (
                        <>
                            <button
                                onClick={handleExportEnv}
                                title="Exportar como .env"
                                className="flex items-center gap-2 text-sm font-medium h-9 px-3 rounded-lg bg-zinc-900 border border-zinc-800 hover:border-zinc-700 text-zinc-300 transition"
                            >
                                <FileDown size={15} /> .env
                            </button>
                            <button
                                onClick={handleImportEnv}
                                title="Importar un .env"
                                className="flex items-center justify-center h-9 w-9 rounded-lg bg-zinc-900 border border-zinc-800 hover:border-zinc-700 text-zinc-300 transition"
                            >
                                <FileUp size={15} />
                            </button>
                        </>
                    )}

                    <CopyMenu item={selectedItem} />

                    {isServer && (
                        <button
                            onClick={handleConnectSsh}
                            disabled={connecting}
                            title="Conectar por SSH"
                            className="flex items-center gap-2 text-sm font-medium h-9 px-3 rounded-lg transition disabled:opacity-40 disabled:cursor-not-allowed bg-zinc-900 border border-zinc-800 hover:border-emerald-600 hover:text-emerald-500 text-zinc-300"
                        >
                            <Terminal size={15} />
                            {connecting ? "Conectando..." : "SSH"}
                        </button>
                    )}

                    <div className="flex-1" />

                    <button
                        onClick={handleSave}
                        disabled={!dirty || saving}
                        title="Guardar cambios"
                        className="flex items-center gap-2 text-sm font-medium h-9 px-4 rounded-lg transition disabled:opacity-40 disabled:cursor-not-allowed bg-blue-600 hover:bg-blue-500 text-white"
                    >
                        <Save size={15} />
                        {saving ? "Guardando..." : "Guardar"}
                    </button>

                    <button
                        onClick={handleDelete}
                        title="Eliminar elemento"
                        className="h-9 w-9 rounded-lg flex items-center justify-center text-zinc-500 hover:text-red-500 hover:bg-zinc-900 border border-transparent hover:border-zinc-800 transition"
                    >
                        <Trash2 size={15} />
                    </button>

                </div>

            </div>

            <div className="max-w-4xl px-8 py-8">

                {draft.fields.map((field) => (
                    <FieldEditor
                        key={field.id}
                        field={field}
                        onChange={(value) => changeField(field.id, value)}
                        onRemove={() => handleRemoveField(field)}
                    />
                ))}

                {!addingField ? (
                    <button
                        onClick={openAddField}
                        className="flex items-center gap-2 text-sm text-zinc-400 hover:text-white h-9 px-3 rounded-lg border border-dashed border-zinc-800 hover:border-zinc-700 transition-colors"
                    >
                        <Plus size={15} />
                        Añadir campo
                    </button>
                ) : (
                    <form
                        onSubmit={handleAddField}
                        className="bg-zinc-900 border border-zinc-800 rounded-lg p-3"
                    >
                        <div className="flex items-center gap-2">
                            <input
                                autoFocus
                                value={newFieldLabel}
                                onChange={(e) => setNewFieldLabel(e.target.value)}
                                placeholder="Nombre del campo (ej: DATABASE_URL)"
                                className="flex-1 h-9 bg-zinc-950 border border-zinc-800 rounded-lg px-3 text-sm text-white outline-none focus:border-blue-500"
                            />
                            <button
                                type="submit"
                                disabled={fieldBusy}
                                className="h-9 px-3 rounded-lg bg-blue-600 hover:bg-blue-500 disabled:opacity-40 text-white text-sm font-medium transition-colors"
                            >
                                {fieldBusy ? "..." : "Añadir"}
                            </button>
                            <button
                                type="button"
                                onClick={() => setAddingField(false)}
                                className="h-9 px-3 rounded-lg text-zinc-400 hover:text-white text-sm transition-colors"
                            >
                                Cancelar
                            </button>
                        </div>
                        <label className="flex items-center gap-2 mt-2.5 text-xs text-zinc-400 select-none cursor-pointer">
                            <input
                                type="checkbox"
                                checked={newFieldHidden}
                                onChange={(e) => setNewFieldHidden(e.target.checked)}
                                className="accent-blue-600"
                            />
                            Valor sensible (se oculta y se borra del portapapeles al copiar)
                        </label>
                        {fieldError && (
                            <div className="mt-2 text-xs text-red-400">{fieldError}</div>
                        )}
                    </form>
                )}

                <div className="mt-8">
                    <label className="block text-sm text-zinc-400 mb-2">Notas</label>
                    <textarea
                        rows={8}
                        value={draft.notes}
                        onChange={(e) => changeNotes(e.target.value)}
                        className="w-full bg-zinc-900 border border-zinc-800 rounded-lg px-4 py-3 text-sm text-white resize-none outline-none transition-colors focus:border-blue-500"
                    />
                </div>

            </div>

        </section>

    );

}
