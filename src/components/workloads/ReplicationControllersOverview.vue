<template>
    <DataTable :value="replicationControllers" @row-click="onRowClick" :paginator="true" :rows="10">
        <template #empty>{{ $t('replicationcontrollers.table.no_results') }}</template>
        <template #loading>{{ $t('replicationcontrollers.table.loading') }}</template>
        <Column field="name" header="Name" />
        <Column field="namespace" header="Namespace" />
        <Column field="desired_replicas" header="Desired" />
        <Column field="replicas" header="Current" />
        <Column field="selector" header="Selector" />
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
        const replicationControllers = ref([]);
        const selectedReplicationController = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchReplicationControllers();
            } catch (error) {
                console.error("Error fetching replication controllers data:", error);
            }
        });

        const fetchReplicationControllers = async () => {
            const response = await invoke("get_replication_controllers_command", { id: clusterConnectionId.value });
            replicationControllers.value = response.data;
        };

        const onRowClick = (e) => {
            selectedReplicationController.value = e.data;
        };

        return {
            replicationControllers,
            fetchReplicationControllers,
            onRowClick,
            selectedReplicationController,
        };
    },
};
</script>
