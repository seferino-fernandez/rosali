<template>
    <DataTable :value="ingresses" @row-click="onRowClick" :paginator="true" :rows="10">
        <template #empty>{{ $t('ingresses.table.no_results') }}</template>
        <template #loading>{{ $t('ingresses.table.loading') }}</template>
        <Column field="name" header="Name" />
        <Column field="namespace" header="Namespace" />
        <Column field="rules" header="Rules" />
        <Column field="load_balancer" header="Load Balancer" />
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
        const ingresses = ref([]);
        const selectedIngress = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchIngresses();
            } catch (error) {
                console.error("Error fetching ingresses data:", error);
            }
        });

        const fetchIngresses = async () => {
            const response = await invoke("get_ingresses_command", { id: clusterConnectionId.value });
            ingresses.value = response.data;
        };

        const onRowClick = (e) => {
            selectedIngress.value = e.data;
        };

        return {
            ingresses,
            fetchIngresses,
            onRowClick,
            selectedIngress,
        };
    },
};
</script>
