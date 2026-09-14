import { createContext, useContext, useEffect, useState } from "react";

import {
    getNodes,
    getItemsByNode,
    createNode as createNodeService,
    createItem as createItemService,
} from "../services/vault";

const VaultContext = createContext(null);

export function VaultProvider({ children }) {

    const [loading, setLoading] = useState(true);

    const [nodes, setNodes] = useState([]);

    const [items, setItems] = useState([]);

    const [selectedNode, setSelectedNode] = useState(null);

    const [selectedItem, setSelectedItem] = useState(null);

    // Modal state
    const [createNodeModal, setCreateNodeModal] = useState({ open: false, parentId: null });

    const [createItemModal, setCreateItemModal] = useState({ open: false, nodeId: null });

    async function reloadNodes() {

        const data = await getNodes();

        setNodes(data);

        return data;

    }

    async function loadItems(nodeId) {

        if (!nodeId) {

            setItems([]);

            return [];

        }

        const data = await getItemsByNode(nodeId);

        setItems(data);

        return data;

    }

    async function initialize() {

        try {

            setLoading(true);

            const data = await getNodes();

            setNodes(data);

        } finally {

            setLoading(false);

        }

    }

    async function createNodeAction(name, parentId, nodeType) {

        const node = await createNodeService(name, parentId, nodeType);

        await reloadNodes();

        setCreateNodeModal({ open: false, parentId: null });

        return node;

    }

    async function createItemAction(nodeId, title, itemType) {

        const item = await createItemService(nodeId, title, itemType);

        await loadItems(nodeId);

        setCreateItemModal({ open: false, nodeId: null });

        setSelectedItem(item);

        return item;

    }

    function openCreateNodeModal(parentId = null) {

        setCreateNodeModal({ open: true, parentId });

    }

    function openCreateItemModal(nodeId) {

        setCreateItemModal({ open: true, nodeId });

    }

    // Navega directo a un item encontrado por el buscador: selecciona su
    // carpeta, carga los items de esa carpeta, y abre el item en el editor.
    async function selectSearchResult(item) {

        const node = nodes.find(n => n.id === item.node_id) || null;

        setSelectedNode(node);

        await loadItems(item.node_id);

        setSelectedItem(item);

    }

    useEffect(() => {

        initialize();

    }, []);

    useEffect(() => {

        if (!selectedNode) {

            setItems([]);

            return;

        }

        loadItems(selectedNode.id);

    }, [selectedNode]);

    const value = {

        loading,

        nodes,

        items,

        selectedNode,

        selectedItem,

        setSelectedNode,

        setSelectedItem,

        reloadNodes,

        loadItems,

        createNodeAction,

        createItemAction,

        selectSearchResult,

        createNodeModal,

        createItemModal,

        openCreateNodeModal,

        openCreateItemModal,

        closeCreateNodeModal: () => setCreateNodeModal({ open: false, parentId: null }),

        closeCreateItemModal: () => setCreateItemModal({ open: false, nodeId: null }),

    };

    return (

        <VaultContext.Provider value={value}>

            {children}

        </VaultContext.Provider>

    );

}

export function useVault() {

    return useContext(VaultContext);

}