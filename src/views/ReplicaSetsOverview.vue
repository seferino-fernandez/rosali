<template>
    <DataTable :value="replicaSets" @row-click="onRowClick" :paginator="true" :rows="10">
        <template #empty>{{ $t('replicasets.table.no_results') }}</template>
        <template #loading>{{ $t('replicasets.table.loading') }}</template>
        <Column field="name" header="Name" />
        <Column field="namespace" header="Namespace" />
        <Column field="desired" header="Desired" />
        <Column field="current" header="Current" />
        <Column field="ready" header="Ready" />
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
        const replicaSets = ref([]);
        const selectedReplicaSet = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchReplicaSets();
            } catch (error) {
                console.error("Error fetching replica sets data:", error);
            }
        });

        const fetchReplicaSets = async () => {
            const response = await invoke("get_replicasets_command", { id: clusterConnectionId.value });
            replicaSets.value = response.data;
        };

        const onRowClick = (e) => {
            selectedReplicaSet.value = e.data;
        };

        return {
            replicaSets,
            fetchReplicaSets,
            onRowClick,
            selectedReplicaSet,
        };
    },
};
</script>
