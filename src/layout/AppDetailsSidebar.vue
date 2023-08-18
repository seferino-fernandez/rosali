<template>
    <div v-show="isVisible" class="justify-content-center surface-card border-round p-1 mb-2">
        <Sidebar :visible="isVisible" @update:visible="hideDetails" class="w-full md:w-5" position="right" :pt="{
            header: { class: 'surface-ground' },
            headerContent: { class: 'flex w-full align-items-center justify-content-between' },
        }">
            <template #header>
                <div class="flex">
                    <span class="font-bold mr-1">{{ resource.kind }}:</span>
                    <span class="font-semibold">{{ resource.metadata.name }}</span>
                </div>
                <!-- TODO: Implement Actions for Resources -->
                <div class="flex">
                    <!-- If resource type is Pods, show a button to view logs -->
                    <Button v-if="resource.kind === 'Pod'" v-tooltip="$t('actions.view_logs')" icon="pi pi-external-link"
                        class="p-sidebar-icon p-link p-mr-5 mx-1" @click="viewLogs" />
                    <!-- <Button v-tooltip="$t('actions.edit')" icon="pi pi-file-edit"
                        class="p-sidebar-icon p-link p-mr-5 mx-1" />
                    <Button v-tooltip="$t('actions.delete')" icon="pi pi-trash" class="p-sidebar-icon p-link p-mr-5 mx-1" /> -->
                </div>
            </template>
            <TabView>
                <TabPanel :header="$t('kubernetes.meta.label')">
                    <div class="flex flex-column">
                        <div>
                            <span class="font-bold mr-1">{{ $t('kubernetes.meta.created') }}:</span>
                            <span>{{ resource.metadata.creationTimestamp }}</span>
                        </div>
                        <Divider type="dashed" />
                        <div>
                            <span class="font-bold mr-1">{{ $t('kubernetes.meta.name', 1) }}:</span>
                            <span>{{ resource.metadata.name }}</span>
                        </div>
                        <Divider type="dashed" />
                        <div>
                            <span class="font-bold mr-1">{{ $t('kubernetes.namespace', 1) }}:</span>
                            <span>{{ resource.metadata.namespace }}</span>
                        </div>

                        <Divider type="dashed" />
                        <div>
                            <span class="font-bold mr-1">{{ $t('kubernetes.label', 2) }}:</span>
                            <div v-for="(value, key) in resource.metadata.labels" :key="key">
                                <span>{{ key }}: {{ value }}</span>
                            </div>
                        </div>

                        <Divider type="dashed" />
                        <div>
                            <span class="font-bold mr-1">{{ $t('kubernetes.annotation', 2) }}:</span>
                            <div v-for="(value, key) in resource.metadata.annotations" :key="key">
                                <span>{{ key }}: {{ value }}</span>
                            </div>
                        </div>

                        <Divider type="dashed" />
                        <div>
                            <span class="font-bold mr-1">{{ $t('kubernetes.meta.owner_reference', 2) }}:</span>
                            <div v-for="(value, index) in resource.metadata.ownerReferences" :key="index">
                                <span>{{ value.kind }}: {{ value.name }}</span>
                            </div>
                        </div>
                    </div>
                </TabPanel>
            </TabView>
        </Sidebar>
    </div>
</template>
  
<script setup>
import { ref, computed } from "vue";
import { useRoute } from "vue-router";
import { invoke } from "@tauri-apps/api";

import Sidebar from 'primevue/sidebar';
import Button from 'primevue/button';
import TabView from 'primevue/tabview';
import TabPanel from 'primevue/tabpanel';
import Divider from 'primevue/divider';

const props = defineProps({
    resource: {
        type: Object,
        default: () => { },
    },
    hideDetails: {
        type: Function,
        required: true,
    },
    showDetailsSidebar: {
        type: Boolean,
        required: true,
    }
})
const route = useRoute();

const isVisible = computed(() => props.showDetailsSidebar);

async function viewLogs() {
    try {
        await invoke("view_logs_window_command", {
            id: route.params.id,
            podName: props.resource.metadata.name,
            namespace: props.resource.metadata.namespace,
        });
    } catch (error) {
        console.error("Error creating logs window", error);
    }
}

</script>
