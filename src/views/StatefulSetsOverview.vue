<template>
    <DataTable :value="statefulSets" @row-click="onRowClick" :paginator="true" :rows="10">
        <template #empty>{{ $t('statefulsets.table.no_results') }}</template>
        <template #loading>{{ $t('statefulsets.table.loading') }}</template>
        <Column field="name" header="Name" />
        <Column field="namespace" header="Namespace" />
        <Column field="ready" header="Ready" />
        <Column field="service" header="Service" />
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
        const statefulSets = ref([]);
        const selectedStatefulSet = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchStatefulSets();
            } catch (error) {
                console.error("Error fetching stateful sets data:", error);
            }
        });

        const fetchStatefulSets = async () => {
            const response = await invoke("get_statefulsets_command", { id: clusterConnectionId.value });
            statefulSets.value = response.data;
        };

        const onRowClick = (e) => {
            selectedStatefulSet.value = e.data;
        };

        return {
            statefulSets,
            fetchStatefulSets,
            onRowClick,
            selectedStatefulSet,
        };
    },
};
</script>