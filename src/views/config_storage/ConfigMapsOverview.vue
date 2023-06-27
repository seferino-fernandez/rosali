<template>
    <DataTable :value="configmaps" @row-click="onRowClick">
        <template #empty>{{ $t('configmaps.table.no_results') }}</template>
        <template #loading>{{ $t('configmaps.table.loading') }}</template>
        <Column field="name" header="Name" />
        <Column field="namespace" header="Namespace" />
        <Column field="keys" header="Keys" />
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
        const configmaps = ref([]);
        const selectedConfigmap = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchConfigMaps();
            } catch (error) {
                console.error("Error fetching configmaps data:", error);
            }
        });

        const fetchConfigMaps = async () => {
            const response = await invoke("get_configmaps_command", { id: clusterConnectionId.value });
            configmaps.value = response.data;
        };

        const onRowClick = (e) => {
            selectedConfigmap.value = e.data;
        };

        return {
            configmaps,
            fetchConfigMaps,
            onRowClick,
            selectedConfigmap,
        };
    },
};
</script>
