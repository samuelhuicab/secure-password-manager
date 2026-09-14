import { useCallback, useEffect, useState } from "react";

import { RefreshCw, Copy, Check, Wand2 } from "lucide-react";

import Modal from "../ui/Modal";
import { generateSecret, copySecret } from "../../services/dev";

const KINDS = [
    { id: "password", label: "Contraseña" },
    { id: "passphrase", label: "Frase" },
    { id: "hex", label: "Hex" },
    { id: "base64", label: "Base64" },
    { id: "uuid", label: "UUID" },
];

function entropyLabel(bits) {
    if (bits < 50) return { text: "Débil", color: "text-orange-400" };
    if (bits < 75) return { text: "Aceptable", color: "text-yellow-400" };
    if (bits < 110) return { text: "Fuerte", color: "text-lime-400" };
    return { text: "Excelente", color: "text-emerald-400" };
}

export default function SecretGenerator({ onClose, onUse }) {

    const [kind, setKind] = useState("password");
    const [length, setLength] = useState(20);
    const [words, setWords] = useState(6);
    const [uppercase, setUppercase] = useState(true);
    const [digits, setDigits] = useState(true);
    const [symbols, setSymbols] = useState(true);
    const [avoidAmbiguous, setAvoidAmbiguous] = useState(true);

    const [result, setResult] = useState({ value: "", entropyBits: 0 });
    const [copied, setCopied] = useState(false);

    const regenerate = useCallback(async () => {
        try {
            const r = await generateSecret({
                kind,
                length: Number(length),
                words: Number(words),
                uppercase,
                digits,
                symbols,
                avoidAmbiguous,
            });
            setResult(r);
            setCopied(false);
        } catch (err) {
            setResult({ value: String(err), entropyBits: 0 });
        }
    }, [kind, length, words, uppercase, digits, symbols, avoidAmbiguous]);

    useEffect(() => {
        regenerate();
    }, [regenerate]);

    async function handleCopy() {
        await copySecret(result.value, 20);
        setCopied(true);
        setTimeout(() => setCopied(false), 1500);
    }

    const ent = entropyLabel(result.entropyBits);
    const isChars = kind === "password" || kind === "hex" || kind === "base64";

    return (

        <Modal title="Generador de secretos" onClose={onClose}>

            <div className="flex flex-wrap gap-1.5 mb-5">
                {KINDS.map((k) => (
                    <button
                        key={k.id}
                        onClick={() => setKind(k.id)}
                        className={`px-3 h-8 rounded-lg text-xs font-medium transition-colors ${
                            kind === k.id
                                ? "bg-blue-600 text-white"
                                : "bg-zinc-900 border border-zinc-800 text-zinc-400 hover:text-white"
                        }`}
                    >
                        {k.label}
                    </button>
                ))}
            </div>

            <div className="bg-zinc-900 border border-zinc-800 rounded-lg p-4 mb-4">
                <div className="text-white text-sm break-all font-mono min-h-[2.5rem] flex items-center">
                    {result.value}
                </div>
                <div className="flex items-center justify-between mt-3 pt-3 border-t border-zinc-800">
                    {kind !== "uuid" ? (
                        <span className={`text-xs ${ent.color}`}>
                            ~{Math.round(result.entropyBits)} bits · {ent.text}
                        </span>
                    ) : <span />}
                    <div className="flex gap-1">
                        <button
                            onClick={regenerate}
                            title="Regenerar"
                            className="w-8 h-8 rounded-md flex items-center justify-center text-zinc-400 hover:text-white hover:bg-zinc-800 transition-colors"
                        >
                            <RefreshCw size={15} />
                        </button>
                        <button
                            onClick={handleCopy}
                            title="Copiar"
                            className="w-8 h-8 rounded-md flex items-center justify-center text-zinc-400 hover:text-white hover:bg-zinc-800 transition-colors"
                        >
                            {copied ? <Check size={15} className="text-emerald-500" /> : <Copy size={15} />}
                        </button>
                    </div>
                </div>
            </div>

            {kind !== "uuid" && (
                <div className="space-y-3 mb-5">
                    {isChars && (
                        <label className="flex items-center justify-between text-sm text-zinc-400">
                            <span>Longitud: {length}</span>
                            <input
                                type="range"
                                min={kind === "password" ? 8 : 8}
                                max={kind === "password" ? 64 : 64}
                                value={length}
                                onChange={(e) => setLength(e.target.value)}
                                className="w-40 accent-blue-600"
                            />
                        </label>
                    )}
                    {kind === "passphrase" && (
                        <label className="flex items-center justify-between text-sm text-zinc-400">
                            <span>Palabras: {words}</span>
                            <input
                                type="range"
                                min={3}
                                max={10}
                                value={words}
                                onChange={(e) => setWords(e.target.value)}
                                className="w-40 accent-blue-600"
                            />
                        </label>
                    )}
                    {(kind === "password" || kind === "passphrase") && (
                        <div className="flex flex-wrap gap-x-4 gap-y-2">
                            {kind === "password" && (
                                <>
                                    <Toggle label="Mayúsculas" checked={uppercase} onChange={setUppercase} />
                                    <Toggle label="Dígitos" checked={digits} onChange={setDigits} />
                                    <Toggle label="Símbolos" checked={symbols} onChange={setSymbols} />
                                    <Toggle label="Sin ambiguos" checked={avoidAmbiguous} onChange={setAvoidAmbiguous} />
                                </>
                            )}
                            {kind === "passphrase" && (
                                <>
                                    <Toggle label="Sufijo numérico" checked={digits} onChange={setDigits} />
                                    <Toggle label="Capitalizar" checked={uppercase} onChange={setUppercase} />
                                </>
                            )}
                        </div>
                    )}
                </div>
            )}

            {onUse && (
                <button
                    onClick={() => onUse(result.value)}
                    className="w-full h-10 rounded-lg bg-blue-600 hover:bg-blue-500 text-white text-sm font-medium inline-flex items-center justify-center gap-2 transition-colors"
                >
                    <Wand2 size={15} />
                    Usar este valor
                </button>
            )}

        </Modal>

    );

}

function Toggle({ label, checked, onChange }) {
    return (
        <label className="flex items-center gap-2 text-sm text-zinc-400 select-none cursor-pointer">
            <input
                type="checkbox"
                checked={checked}
                onChange={(e) => onChange(e.target.checked)}
                className="accent-blue-600"
            />
            {label}
        </label>
    );
}
