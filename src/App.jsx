import { BrowserRouter, Routes, Route } from "react-router-dom";

import Dashboard from "./pages/Dashboard";
import LockScreen from "./pages/LockScreen";

import { AuthProvider, useAuth } from "./contexts/AuthContext";
import { VaultProvider } from "./contexts/VaultContext";

import useAutoLock from "./hooks/useAutoLock";

import "./App.css";

function Gate() {

    const { status, lock } = useAuth();

    useAutoLock(status === "unlocked", lock);

    if (status === "loading") {
        return <div className="h-screen bg-zinc-950" />;
    }

    if (status !== "unlocked") {
        return <LockScreen />;
    }

    return (
        <VaultProvider>
            <BrowserRouter>
                <Routes>
                    <Route path="/" element={<Dashboard />} />
                </Routes>
            </BrowserRouter>
        </VaultProvider>
    );

}

function App() {
    return (
        <AuthProvider>
            <Gate />
        </AuthProvider>
    );
}

export default App;
