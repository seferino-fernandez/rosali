<template>
    <router-view @context-selected="onContextSelected" @view-logs="onViewLogs" />
</template>
  
<script>
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
};
</script>
  