<template>
    <DataTable :value="clusterRoleBindings" @row-click="onRowClick" :paginator="true" :rows="10">
        <template #empty>{{ $t('clusterrolebindings.table.no_results') }}</template>
        <template #loading>{{ $t('clusterrolebindings.table.loading') }}</template>
        <Column field="name" header="Name" />
        <Column field="bindings" header="Bindings" :body="bindingsTemplate"/>
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
        const clusterRoleBindings = ref([]);
        const selectedClusterRoleBinding = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchClusterRoleBindings();
            } catch (error) {
                console.error("Error fetching cluster role bindings data:", error);
            }
        });

        const fetchClusterRoleBindings = async () => {
            const response = await invoke("get_clusterrole_bindings_command", { id: clusterConnectionId.value });
            clusterRoleBindings.value = response.data;
        };

        const onRowClick = (e) => {
            selectedClusterRoleBinding.value = e.data;
        };

        const bindingsTemplate = (rowData) => {
            return rowData.bindings.join(', ');
        }

        return {
            clusterRoleBindings,
            fetchClusterRoleBindings,
            onRowClick,
            selectedClusterRoleBinding,
            bindingsTemplate,
        };
    },
};
</script>
