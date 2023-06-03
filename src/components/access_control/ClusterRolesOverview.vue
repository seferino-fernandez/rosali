<template>
    <DataTable :value="clusterRoles" @row-click="onRowClick" :paginator="true" :rows="10">
        <template #empty>{{ $t('clusterroles.table.no_results') }}</template>
        <template #loading>{{ $t('clusterroles.table.loading') }}</template>
        <Column field="name" header="Name" />
        <Column field="labels" header="Labels" />
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
        const clusterRoles = ref([]);
        const selectedClusterRole = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchClusterRoles();
            } catch (error) {
                console.error("Error fetching cluster roles data:", error);
            }
        });

        const fetchClusterRoles = async () => {
            const response = await invoke("get_clusterroles_command", { id: clusterConnectionId.value });
            clusterRoles.value = response.data;
        };

        const onRowClick = (e) => {
            selectedClusterRole.value = e.data;
        };

        return {
            clusterRoles,
            fetchClusterRoles,
            onRowClick,
            selectedClusterRole,
        };
    },
};
</script>
