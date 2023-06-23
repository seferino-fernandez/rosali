<template>
    <DataTable :value="persistentVolumeClaims" @row-click="onRowClick">
        <template #empty>{{ $t('persistent_volume_claims.table.no_results') }}</template>
        <template #loading>{{ $t('persistent_volume_claims.table.loading') }}</template>
        <Column field="name" header="Name" />
        <Column field="namespace" header="Namespace" />
        <Column field="storage_class" header="Storage Class" />
        <Column field="size" header="Size" />
        <Column field="pods" header="Pods" />
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
        const persistentVolumeClaims = ref([]);
        const selectedPersistentVolumeClaim = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchPersistentVolumeClaims();
            } catch (error) {
                console.error("Error fetching persistent volume claims data:", error);
            }
        });

        const fetchPersistentVolumeClaims = async () => {
            const response = await invoke("get_persistent_volumn_claims_command", { id: clusterConnectionId.value });
            persistentVolumeClaims.value = response.data;
        };

        const onRowClick = (e) => {
            selectedPersistentVolumeClaim.value = e.data;
        };

        return {
            persistentVolumeClaims,
            fetchPersistentVolumeClaims,
            onRowClick,
            selectedPersistentVolumeClaim,
        };
    },
};
</script>
