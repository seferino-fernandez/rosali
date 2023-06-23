<template>
    <DataTable :value="namespaces" @row-click="onRowClick">
        <template #empty>{{ $t('namespaces.table.no_results') }}</template>
        <template #loading>{{ $t('namespaces.table.loading') }}</template>
        <Column field="name" header="Name" />
        <Column field="labels" header="Labels" />
        <Column field="age" header="Age" />
        <Column field="status" header="Status" />
    </DataTable>
</template>

<script>
import { ref, onMounted } from "vue";
import { useRoute } from "vue-router";
import { invoke } from "@tauri-apps/api";
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';

export default {
    components: {
        DataTable,
        Column,
    },
    setup() {
        const route = useRoute();
        const namespaces = ref([]);
        const selectedNamespace = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchNamespaces();
            } catch (error) {
                console.error("Error fetching namespaces data:", error);
            }
        });

        const fetchNamespaces = async () => {
            const response = await invoke("get_namespaces_command", { id: clusterConnectionId.value });
            namespaces.value = response.data;
        };

        const onRowClick = (e) => {
            selectedNamespace.value = e.data;
        };

        return {
            namespaces,
            fetchNamespaces,
            onRowClick,
            selectedNamespace,
        };
    },
};
</script>

<style scoped></style>
