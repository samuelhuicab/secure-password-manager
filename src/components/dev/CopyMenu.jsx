import { useEffect, useState } from "react";
import { Link2, Check } from "lucide-react";

import { copyConnectionString } from "../../services/dev";
import { getSettings } from "../../services/auth";

/**
 * Botón que copia la cadena de conexión de un item (Base de datos o Servidor):
 * `postgresql://…`, `mysql://…`, `mongodb://…` o `ssh user@host -p port`.
 */
export default function CopyMenu({ item }) {

    const [done, setDone] = useState(false);
    const [clearAfter, setClearAfter] = useState(20);

    useEffect(() => {
        getSettings()
            .then((s) => setClearAfter(s.clipboardClearSeconds ?? 20))
            .catch(() => {});
    }, []);

    const canConn =
        item.item_type === "Database" || item.item_type === "Server";

    if (!canConn) return null;

    async function copy() {
        try {
            await copyConnectionString(item.id, clearAfter);
            setDone(true);
            setTimeout(() => setDone(false), 1500);
        } catch (err) {
            alert(String(err));
        }
    }

    return (
        <button
            onClick={copy}
            title="Copiar cadena de conexión"
            className="flex items-center gap-2 text-sm font-medium h-9 px-3 rounded-lg bg-zinc-900 border border-zinc-800 hover:border-zinc-700 text-zinc-300 transition"
        >
            {done ? <Check size={15} className="text-emerald-500" /> : <Link2 size={15} />}
            Conexión
        </button>
    );

}
