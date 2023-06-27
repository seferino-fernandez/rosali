<template>
  <div>
    <DataTable v-model:filters="filters" :value="contexts" @row-click="onRowClick" :selection-mode="'single'"
      filterDisplay="row" :globalFilterFields="['name', 'path', 'cluster', 'user', 'namespace']" scrollable
      scrollHeight="flex">
      <template #header>
        <div class="flex justify-content-between">
          <span class="">{{ $t('kubeconfig.table.header') }}</span>
          <span class="p-input-icon-left">
            <i class="pi pi-search" />
            <InputText v-model="filters['global'].value" placeholder="Search" />
          </span>
        </div>
      </template>
      <template #empty>{{ $t('kubeconfig.table.no_results') }}</template>
      <template #loading>{{ $t('kubeconfig.table.loading') }}</template>
      <Column field="name" filterField="name" :header="$t('kubeconfig.table.name')">
        <template #body="{ data }">
          {{ data.name }}
        </template>
      </Column>
      <Column field="path" filterField="path" :header="$t('kubeconfig.table.path')">
        <template #body="{ data }">
          {{ data.path }}
        </template>
      </Column>
      <Column field="cluster" filterField="cluster" :header="$t('kubeconfig.table.cluster')">
        <template #body="{ data }">
          {{ data.cluster }}
        </template>
      </Column>
      <Column field="user" filterField="user" :header="$t('kubeconfig.table.user')">
        <template #body="{ data }">
          {{ data.user }}
        </template>
      </Column>
      <Column field="namespace" filterField="namespace" :header="$t('kubeconfig.table.namespace')">
        <template #body="{ data }">
          {{ data.namespace }}
        </template>
      </Column>
    </DataTable>
  </div>
</template>
  
<script>
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import { FilterMatchMode } from 'primevue/api';
import { onMounted, ref } from "vue";
import { useRouter, NavigationFailureType, isNavigationFailure } from "vue-router";
import { invoke } from "@tauri-apps/api";

export default {
  components: {
    DataTable,
    Column,
    Button,
    InputText,
  },
  props: {
    onContextSelected: {
      type: Function,
      required: true,
    },
  },
  setup(props) {
    const router = useRouter();
    const contexts = ref([]);

    onMounted(async () => {
      try {
        const defaultKubeConfigData = await invoke("find_default_config_command");
        contexts.value = defaultKubeConfigData.data;
      } catch (error) {
        console.error('Error fetching kubeconfig data:', error);
      }
    });

    async function onRowClick(event, rowData) {
      let contextName = event.data.name;
      let contextPath = event.data.path;
      let params = {};
      props.onContextSelected(contextPath, contextName, "ClusterOverview", params);
    }

    return {
      contexts,
      onRowClick,
      filters: {
        global: { value: null, matchMode: FilterMatchMode.CONTAINS },
        name: { value: null, matchMode: FilterMatchMode.STARTS_WITH },
        cluster: { value: null, matchMode: FilterMatchMode.STARTS_WITH },
        user: { value: null, matchMode: FilterMatchMode.STARTS_WITH },
        namespace: { value: null, matchMode: FilterMatchMode.STARTS_WITH },
      },
      loading: true,
    };
  },
};
</script>
<style scoped>
</style>
  