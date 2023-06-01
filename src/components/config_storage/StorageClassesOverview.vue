<template>
    <DataTable :value="storageClasses" @row-click="onRowClick" :paginator="true" :rows="10">
        <template #empty>{{ $t('storageclasses.table.no_results') }}</template>
        <template #loading>{{ $t('storageclasses.table.loading') }}</template>
        <Column field="name" header="Name" />
        <Column field="provisioner" header="Provisioner" />
        <Column field="reclaim_policy" header="Reclaim Policy" />
        <Column field="default" header="Default">
            <template #body="slotProps">
                <span>{{ slotProps.data.default ? 'True' : 'False' }}</span>
            </template>
        </Column>
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
        const storageClasses = ref([]);
        const selectedStorageClass = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchStorageClasses();
            } catch (error) {
                console.error("Error fetching storage classes data:", error);
            }
        });

        const fetchStorageClasses = async () => {
            const response = await invoke("get_storage_classes_command", { id: clusterConnectionId.value });
            storageClasses.value = response.data;
        };

        const onRowClick = (e) => {
            selectedStorageClass.value = e.data;
        };

        return {
            storageClasses,
            fetchStorageClasses,
            onRowClick,
            selectedStorageClass,
        };
    },
};
</script>
