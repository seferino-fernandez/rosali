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
                            <Button class="tab-name-button" text size="small">{{ item.label }}</Button>
                            <Button class="tab-cancel-button" icon="pi pi-times" text size="small"
                                @click.stop="closeTab(item)" />
                        </div>
                    </template>
                </TabMenu>
                <Divider layout="vertical" />
                <TabMenu class="log-tab-menu" :model="logTabs" @tabChange="onLogTabChange">
                    <template #item="{ item }">
                        <div class="p-buttonset">
                            <Button class="tab-name-button" text size="small">{{ item.label }}</Button>
                            <Button class="tab-cancel-button" icon="pi pi-times" text size="small"
                                @click.stop="closeLogTab(item)" />
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
import Dropdown from 'primevue/dropdown';
import Divider from 'primevue/divider';
import Toolbar from 'primevue/toolbar';
import { useRouter } from "vue-router";
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api";

export default {
    components: {
        TabMenu,
        Button,
        Dropdown,
        Divider,
        Toolbar,
    },
    props: {
        tabs: {
            type: Array,
            default: () => [],
        },
        logTabs: {
            type: Array,
            default: () => [],
        },
    },
    setup(props) {
        const router = useRouter();
        const selectedLogTab = ref(null);

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

            const logTabIndex = props.logTabs.findIndex(
                (t) => t.params.id === tab.params.id
            );
            if (logTabIndex > -1) {
                props.logTabs.splice(logTabIndex, 1);
            }
        }

        function onLogTabChange(event) {
            router.push(props.logTabs[event.index]);
        };

        async function closeLogTab(tab) {
            const logTabIndex = props.logTabs.findIndex(
                (t) => t.label === tab.label
            );

            if (logTabIndex > -1) {
                props.logTabs.splice(logTabIndex, 1);
                if (props.logTabs.length > 0) {
                    // If there are still logTabs remaining, navigate to the last one
                    router.push(props.logTabs[props.logTabs.length - 1]);
                } else if (props.tabs.length > 0) {
                    // If there are no more logTabs but there are still main tabs, navigate to the last tab
                    router.push(props.tabs[props.tabs.length - 1]);
                } else {
                    // If there are no more tabs at all, go to the main page
                    goToKubeconfigContexts();
                }
            }
        }

        return {
            goToKubeconfigContexts,
            onTabChange,
            closeTab,
            selectedLogTab,
            onLogTabChange,
            closeLogTab,
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

.log-tab-menu .p-tabmenu-nav {
    flex-wrap: nowrap;
    overflow-x: auto;
    white-space: nowrap;
}
</style>
