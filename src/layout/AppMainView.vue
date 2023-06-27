<template>
    <div :class="appMainViewClass">
      <router-view @context-selected="onContextSelected" @view-logs="onViewLogs" />
    </div>
  </template>
  
  <script>
  import { computed } from 'vue';
  import { useRoute } from 'vue-router';
  
  export default {
    props: {
      onContextSelected: {
        type: Function,
        required: true,
      },
      onViewLogs: {
        type: Function,
        required: true,
      },
    },
    setup() {
      const route = useRoute();
      const appMainViewClass = computed(() => {
        return route.name === "KubeconfigContexts" ? "app-main-view-no-sidebar" : "app-main-view";
      });
      
      return { appMainViewClass };
    }
  };
  </script>
  
  <style scoped>
  .app-main-view {
    display: flex;
    flex: 1;
    flex-direction: column;
    margin-left: 190px;
    overflow: auto;
  }
  
  .app-main-view-no-sidebar {
    display: flex;
    flex: 1;
    flex-direction: column;
    overflow: auto;
  }
  </style>
  
  