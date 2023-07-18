<template>
    <header class="app-topbar">
        <!-- TODO: Implement Settings Page -->
        <!-- <Button class="flex-none m-1" icon="pi pi-cog" /> -->
        <Button class="flex-none m-1" icon="pi pi-home" @click="goToKubeconfigContexts" raised />
        <TabMenu class="flex-grow-1" :model="tabs" @tabChange="onTabChange">
            <template #item="{ item }">
                <div class="p-buttonset">
                    <Button text size="small">{{ item.label }}</Button>
                    <Button icon="pi pi-times" text size="small" @click.stop="closeTab(item)" />
                </div>
            </template>
        </TabMenu>
    </header>
</template>
  
<script setup>
import TabMenu from "primevue/tabmenu";
import Button from 'primevue/button';
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api";

const props = defineProps({
    tabs: {
        type: Array,
        default: () => [],
    },
})

const router = useRouter();

function goToKubeconfigContexts() {
    router.push({ name: "KubeconfigContexts" });
};
function onTabChange(event) {
    router.push(props.tabs[event.index]);
};

async function closeTab(tab) {
    const tabIndex = props.tabs.findIndex(
        (t) => t.name === tab.name && t.params.id === tab.params.id
    );

    if (tabIndex > -1) {
        if (tab.type === 'cluster') {
            const removeConnectionResponse = await invoke("remove_cluster_connection", { id: tab.id });
        }
        props.tabs.splice(tabIndex, 1);
        if (tabIndex <= 0) {
            goToKubeconfigContexts();
        } else {
            router.push(props.tabs[tabIndex - 1]);
        }
    }
}
</script>
  
<style scoped>
.app-topbar {
    display: flex;
    align-items: center;
    height: 4rem;
    background-color: var(--surface-ground);
    padding: 0 0.25rem;
    top: 0;
    width: 100vw;
    position: fixed;
    z-index: 5;
}
</style>