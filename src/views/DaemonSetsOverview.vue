<template>
    <DataTable :value="daemonSets" @row-click="onRowClick">
        <template #empty>{{ $t('daemonsets.table.no_results') }}</template>
        <template #loading>{{ $t('daemonsets.table.loading') }}</template>
        <Column field="name" header="Name" />
        <Column field="namespace" header="Namespace" />
        <Column field="desired" header="Desired" />
        <Column field="current" header="Current" />
        <Column field="ready" header="Ready" />
        <Column field="up_to_date" header="Up to Date" />
        <Column field="available" header="Available" />
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
        const daemonSets = ref([]);
        const selectedDaemonSet = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchDaemonSets();
            } catch (error) {
                console.error("Error fetching daemon sets data:", error);
            }
        });

        const fetchDaemonSets = async () => {
            const response = await invoke("get_daemonsets_command", { id: clusterConnectionId.value });
            daemonSets.value = response.data;
        };

        const onRowClick = (e) => {
            selectedDaemonSet.value = e.data;
        };

        return {
            daemonSets,
            fetchDaemonSets,
            onRowClick,
            selectedDaemonSet,
        };
    },
};
</script>