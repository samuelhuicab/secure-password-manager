import { useEffect, useState } from "react";
import { Eye, EyeOff, Copy, Check, Wand2, X } from "lucide-react";

import { copySecret } from "../../services/dev";
import { getSettings } from "../../services/auth";
import SecretGenerator from "../dev/SecretGenerator";

export default function FieldEditor({ field, onChange, onRemove }) {

    const [show, setShow] = useState(false);
    const [copied, setCopied] = useState(false);
    const [genOpen, setGenOpen] = useState(false);
    const [clearAfter, setClearAfter] = useState(20);

    useEffect(() => {
        getSettings()
            .then((s) => setClearAfter(s.clipboardClearSeconds ?? 20))
            .catch(() => {});
    }, []);

    async function copy() {
        try {
            await copySecret(field.value, field.hidden ? clearAfter : 0);
        } catch {
            await navigator.clipboard.writeText(field.value);
        }
        setCopied(true);
        setTimeout(() => setCopied(false), 1500);
    }

    return (

        <div className="mb-5">

            <label className="flex items-center justify-between text-sm text-zinc-400 mb-2">
                <span>{field.label}</span>
                {onRemove && (
                    <button
                        onClick={onRemove}
                        title="Quitar campo"
                        className="text-zinc-600 hover:text-red-400 transition-colors"
                    >
                        <X size={14} />
                    </button>
                )}
            </label>

            <div className="flex items-center h-11 bg-zinc-900 border border-zinc-800 rounded-lg overflow-hidden transition-colors focus-within:border-blue-500">

                <input
                    type={field.hidden && !show ? "password" : "text"}
                    value={field.value}
                    onChange={(e) => onChange(e.target.value)}
                    className="flex-1 h-full bg-transparent px-4 text-sm text-white outline-none"
                />

                {field.hidden && (
                    <button
                        onClick={() => setGenOpen(true)}
                        title="Generar"
                        className="px-3 h-full text-zinc-500 hover:text-white hover:bg-zinc-800 transition-colors"
                    >
                        <Wand2 size={16} />
                    </button>
                )}

                {field.hidden && (
                    <button
                        onClick={() => setShow(!show)}
                        title={show ? "Ocultar" : "Mostrar"}
                        className="px-3 h-full text-zinc-500 hover:text-white hover:bg-zinc-800 transition-colors"
                    >
                        {show ? <EyeOff size={16} /> : <Eye size={16} />}
                    </button>
                )}

                <button
                    onClick={copy}
                    title="Copiar"
                    className="px-3 h-full text-zinc-500 hover:text-white hover:bg-zinc-800 transition-colors"
                >
                    {copied ? <Check size={16} className="text-emerald-500" /> : <Copy size={16} />}
                </button>

            </div>

            {genOpen && (
                <SecretGenerator
                    onClose={() => setGenOpen(false)}
                    onUse={(value) => {
                        onChange(value);
                        setGenOpen(false);
                    }}
                />
            )}

        </div>

    );

}
