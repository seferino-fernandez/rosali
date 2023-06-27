<template>
    <DataTable :value="secrets" @row-click="onRowClick">
        <template #empty>{{ $t('secrets.table.no_results') }}</template>
        <template #loading>{{ $t('secrets.table.loading') }}</template>
        <Column field="name" header="Name" />
        <Column field="namespace" header="Namespace" />
        <Column field="labels" header="Labels" />
        <Column field="keys" header="Keys" />
        <Column field="secret_type" header="Type" />
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
        const secrets = ref([]);
        const selectedSecret = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchSecrets();
            } catch (error) {
                console.error("Error fetching secrets data:", error);
            }
        });

        const fetchSecrets = async () => {
            const response = await invoke("get_secrets_command", { id: clusterConnectionId.value });
            secrets.value = response.data;
        };

        const onRowClick = (e) => {
            selectedSecret.value = e.data;
        };

        return {
            secrets,
            fetchSecrets,
            onRowClick,
            selectedSecret,
        };
    },
};
</script>
