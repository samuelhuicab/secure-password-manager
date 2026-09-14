import { Lock } from "lucide-react";

import { useAuth } from "../../contexts/AuthContext";

export default function LockButton() {

    const { lock } = useAuth();

    return (
        <button
            onClick={() => lock()}
            title="Bloquear vault (Ctrl+L)"
            className="
                w-8
                h-8
                rounded-md
                flex
                items-center
                justify-center
                text-zinc-500
                hover:bg-zinc-800
                hover:text-zinc-200
                transition-colors
            "
        >
            <Lock size={13} />
        </button>
    );

}
