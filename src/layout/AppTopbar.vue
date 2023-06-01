<template>
    <header class="app-header">
        <Toolbar>
            <template #start>
                <Button icon="pi pi-home" class="home-button" @click="goToKubeconfigContexts" raised />
            </template>
            <template #center>
                <TabMenu class="top-bar-menu" :model="tabs" @tabChange="onTabChange">
                    <template #item="{ item }">
                        <div class="p-buttonset">
                            <Button class="tab-name-button" text size="small" >{{ item.label }}</Button>
                            <Button class="tab-cancel-button" icon="pi pi-times" text size="small" @click.stop="closeTab(item)" />
                        </div>
                    </template>
                </TabMenu>
            </template>
            <template #end>
                <Button icon="pi pi-search" class="mr-2" />
            </template>
        </Toolbar>
    </header>
</template>
  
<script>
import TabMenu from "primevue/tabmenu";
import Button from 'primevue/button';
import Divider from 'primevue/divider';
import Toolbar from 'primevue/toolbar';
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api";

export default {
    components: {
        TabMenu,
        Button,
        Divider,
        Toolbar,
    },
    props: {
        tabs: {
            type: Array,
            default: () => [],
        },
    },
    setup(props) {
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
                const removeConnectionResponse = await invoke("remove_cluster_connection", { id: tab.id });
                props.tabs.splice(tabIndex, 1);
                if (tabIndex <= 0) {
                    goToKubeconfigContexts();
                } else {
                    router.push(props.tabs[tabIndex - 1]);
                }
            }
        }

        return {
            goToKubeconfigContexts,
            onTabChange,
            closeTab
        };
    },
};
</script>
  
<style scoped>
.app-header {
    display: flex;
    align-items: center;
    width: 100%;
    z-index: 100;
}

.tab-name-button {
    padding-right: 10px;
}

.tab-cancel-button {
    width: 0;
}
</style>
