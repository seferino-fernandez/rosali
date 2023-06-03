<template>
    <DataTable :value="roleBindings" @row-click="onRowClick" :paginator="true" :rows="10">
        <template #empty>{{ $t('rolebindings.table.no_results') }}</template>
        <template #loading>{{ $t('rolebindings.table.loading') }}</template>
        <Column field="name" header="Name" />
        <Column field="namespace" header="Name" />
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
        const roleBindings = ref([]);
        const selectedRoleBinding = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchRoleBindings();
            } catch (error) {
                console.error("Error fetching role bindings data:", error);
            }
        });

        const fetchRoleBindings = async () => {
            const response = await invoke("get_role_bindings_command", { id: clusterConnectionId.value });
            roleBindings.value = response.data;
        };

        const onRowClick = (e) => {
            selectedRoleBinding.value = e.data;
        };

        const bindingsTemplate = (rowData) => {
            return rowData.bindings.join(', ');
        }

        return {
            roleBindings,
            fetchRoleBindings,
            onRowClick,
            selectedRoleBinding,
            bindingsTemplate,
        };
    },
};
</script>
