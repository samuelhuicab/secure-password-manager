export default function StatusBar() {

    return (

        <footer

            className="

                h-8

                border-t

                border-zinc-800

                flex

                items-center

                justify-between

                px-4

                text-xs

                text-zinc-500

                bg-zinc-950

            "

        >

            <span>

                Samuel Huicab Pastrana — Organizador

            </span>

            <div className="flex items-center gap-1.5">

                <span className="w-1.5 h-1.5 rounded-full bg-emerald-500" />

                Vault local · cifrado

            </div>

        </footer>

    );

}