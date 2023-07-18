<template>
    <div class="pod-overview flex flex-column align-items-center">
        <DataTable class="border-round-lg w-full h-full" scrollable scrollHeight="flex" v-model:selection="checkedPods"
            v-model:filters="filters" filterDisplay="menu" :value="pods" @rowSelect="onRowSelect"
            @rowUnselect="onRowUnselect" @rowSelectAll="onRowSelectAll" @rowUnselectAll="onRowUnselectAll"
            @rowClick="onRowClick" stripedRows dataKey="name" :globalFilterFields="['name', 'namespace']">
            <template #header>
                <div class="flex justify-content-between align-items-center">
                    <div>
                        <span class="">{{ $t("kubernetes.pod", 2) }} </span>
                        <span class=""> ({{ pods ? pods.length : 0 }})</span>
                    </div>
                    <div>
                        <Button icon="pi pi-refresh" rounded raised @click="fetchPods" />
                        <span class="p-input-icon-left ml-4">
                            <i class="pi pi-search" />
                            <InputText v-model="filters['global'].value" :placeholder="$t('actions.search')" />
                        </span>
                    </div>
                </div>
            </template>
            <!--
                TODO Implement Bulk Actions
            <Column selectionMode="multiple" headerStyle="width: 3rem"></Column>
            -->
            <Column field="name" sortable :header="$t('kubernetes.meta.name', 1)">
                <!--
                TODO: Implement filters for the Pod Name
                <template #filter="{ filterModel }">
                    <InputText v-model="filterModel.value" type="text" class="p-column-filter"
                        :placeholder="$t('actions.search')" />
                </template>
                -->
                <template #body="slotProps">
                    <span class="font-bold">{{ slotProps.data.name }}</span>
                </template>
            </Column>
            <Column field="namespace" sortable :header="$t('kubernetes.namespace', 1)">
                <!--
                TODO: Implement filters for the Namespace
                 <template #filter="{ filterModel }">
                    <InputText v-model="filterModel.value" type="text" class="p-column-filter"
                        :placeholder="$t('actions.search')" />
                </template>
                -->
            </Column>
            <Column field="status" sortable :header="$t('kubernetes.meta.status.label')">
                <template #body="slotProps">
                    <Tag :value="slotProps.data.status" :severity="getPodSeverity(slotProps.data.status)" />
                </template>
            </Column>
            <Column field="restarts" bodyStyle="justify-content-end" sortable dataType="numeric"
                :header="$t('kubernetes.meta.restart', 2)">
                <template #body="slotProps">
                    <div class="flex align-items-right">
                        <span>{{ slotProps.data.restarts }}</span>
                    </div>
                </template>
            </Column>
            <Column field="kube_object.status.qosClass" sortable :header="$t('kubernetes.meta.qos')" />
            <Column field="age" sortable :header="$t('kubernetes.meta.age')" />
            <!--
                TODO: Implement Actions for the Pods
                <Column headerStyle="width: 5rem; text-align: center" bodyStyle="text-align: center; overflow: visible">
                <template #body>
                    <Button type="button" icon="pi pi-ellipsis-v" rounded />
                </template>
            </Column>
            -->
            <template #footer />
        </DataTable>
        <!-- TODO: Remove 'false' from v-if when Bulk Actions is implemented -->
        <div v-if="false || showBulkActions"
            class="flex fixed z-1 bottom-0 mb-5 border-round p-2 bg-primary justify-content-center align-items-center border-left-1 border-right-1 border-bottom-2 shadow-8">
            <span class="mx-1"> {{ checkedPods ? checkedPods.length : 0 }} selected</span>
            <Divider layout="vertical" type="dashed" />
            <Button v-tooltip="$t('actions.view_logs')" icon="pi pi-external-link"
                class="p-sidebar-icon p-link p-mr-5 mx-1" />
            <Button v-tooltip="$t('actions.edit')" icon="pi pi-file-edit" class="p-sidebar-icon p-link p-mr-5 mx-1" />
            <Button v-tooltip="$t('actions.delete')" icon="pi pi-trash" class="p-sidebar-icon p-link p-mr-5 mx-1" />
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted, inject, computed } from "vue";
import { useRoute } from "vue-router";
import { invoke } from "@tauri-apps/api";
import { useI18n } from "vue-i18n";
import { FilterMatchMode } from "primevue/api";
import Button from "primevue/button";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import InputText from "primevue/inputtext";
import Divider from "primevue/divider";
import Tag from "primevue/tag";

const route = useRoute();
const pods = ref([]);
const checkedPods = ref([]);
const selectedPod = ref({});
const clusterConnectionId = ref(route.params.id);
const showDetails = ref(false);
const showBulkActions = computed(() => checkedPods.value.length > 0);
const filters = ref({
    global: { value: null, matchMode: FilterMatchMode.CONTAINS },
    name: { value: null, matchMode: FilterMatchMode.STARTS_WITH },
    namespace: { value: null, matchMode: FilterMatchMode.STARTS_WITH },
});
const { t } = useI18n();
const emit = defineEmits(["view-logs"]);
const displayDetails = inject('displayDetails');

const speedDialActions = ref([
    {
        label: t("actions.view_logs"),
        icon: "pi pi-file",
        command: () => { },
    },
    {
        label: t("actions.delete"),
        icon: "pi pi-trash",
        command: () => { },
    },
]);

onMounted(fetchPods);

async function fetchPods() {
    try {
        const response = await invoke("get_pods_command", {
            id: clusterConnectionId.value,
        });
        pods.value = response.data;
    } catch (error) {
        console.error("Error fetching pods data:", error);
    }
}

async function viewLogs(e) {
    showDetails.value = false;
    emit("view-logs", {
        connectionId: clusterConnectionId,
        pod: selectedPod,
    });
}

function onRowClick(event) {
    /**
     * * This is done to ignore the onRowClick event when the checkbox is un-selected.
     */
    if (event.originalEvent && event.originalEvent.target.classList && event.originalEvent.target.classList.contains("p-checkbox-icon")) {
        return;
    }
    displayDetails(event.data.kube_object);
}

function onRowSelect(event) {
    if (!checkedPods.value.some((pod) => pod.name === event.data.name)) {
        checkedPods.value.push(event.data);
    }
}

function onRowUnselect(event) {
    checkedPods.value = checkedPods.value.filter(
        (pod) => pod.name !== event.data.name
    );
}

function onRowSelectAll(event) {
    event.data.forEach((selectedPod) => {
        if (!checkedPods.value.some((pod) => pod.name === selectedPod.name)) {
            checkedPods.value.push(selectedPod);
        }
    });
}

function onRowUnselectAll(event) {
    checkedPods.value = [];
}

function initFilters() {
    filters.value = {
        global: { value: null, matchMode: FilterMatchMode.CONTAINS },
        name: { value: null, matchMode: FilterMatchMode.STARTS_WITH },
        namespace: { value: null, matchMode: FilterMatchMode.STARTS_WITH },
    };
}

const getPodSeverity = (podStatus) => {
    switch (podStatus) {
        case 'Running':
            return 'success';
        case 'Terminating':
            return 'warning';
        case 'Pending':
            return 'warning';
        case 'Succeeded':
            return 'info';
        case 'Unknown':
            return 'danger';
        case 'Failed':
            return 'danger';
        default:
            return null;
    }
}

initFilters();

function clearFilter() {
    initFilters();
}
</script>