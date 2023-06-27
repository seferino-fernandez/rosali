<template>
    <DataTable :value="endpoints" @row-click="onRowClick">
        <template #empty>{{ $t('endpoints.table.no_results') }}</template>
        <template #loading>{{ $t('endpoints.table.loading') }}</template>
        <Column field="name" header="Name" />
        <Column field="namespace" header="Namespace" />
        <Column field="endpoints" header="Endpoints" />
        <Column field="age" header="Age" />
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
        const endpoints = ref([]);
        const selectedEndpoint = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchEndpoints();
            } catch (error) {
                console.error("Error fetching endpoints data:", error);
            }
        });

        const fetchEndpoints = async () => {
            const response = await invoke("get_endpoints_command", { id: clusterConnectionId.value });
            endpoints.value = response.data;
        };

        const onRowClick = (e) => {
            selectedEndpoint.value = e.data;
        };

        return {
            endpoints,
            fetchEndpoints,
            onRowClick,
            selectedEndpoint,
        };
    },
};
</script>
