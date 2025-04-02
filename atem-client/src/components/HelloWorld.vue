<script setup lang="ts">

</script>

<template>
  <el-timeline style="max-width: 600px">
    <el-timeline-item timestamp="2024/7/1" placement="top">
      <div>
        <el-tag type="primary" style="margin-right: 5px; width: 60px;">黎创</el-tag>
        <el-tag type="primary" style="margin-right: 5px; width: 60px;">陈思仪</el-tag>
      </div>
    </el-timeline-item>
    <el-timeline-item timestamp="2024/7/25" placement="top">
      <div>
        <el-tag type="primary" style="margin-right: 5px; width: 60px;">曾展华</el-tag>
      </div>
    </el-timeline-item>
    <el-timeline-item timestamp="2024/8/26" placement="top">
      <div>
        <el-tag type="primary" style="margin-right: 5px; width: 60px;">徐绮晴</el-tag>
      </div>
    </el-timeline-item>
    <el-timeline-item timestamp="2024/10/10" placement="top">
      <div>
        <el-tag type="primary" style="margin-right: 5px; width: 60px;">谭俊标</el-tag>
      </div>
    </el-timeline-item>
    <el-timeline-item timestamp="2024/12/2" placement="top">
      <div>
        <el-tag type="primary" style="margin-right: 5px; width: 60px;">杨正英</el-tag>
      </div>
    </el-timeline-item>
  </el-timeline>
</template>


<style scoped>
h1 {
  font-weight: 500;
  font-size: 2.6rem;
  position: relative;
  top: -10px;
}

h3 {
  font-size: 1.2rem;
}

.greetings h1,
.greetings h3 {
  text-align: center;
}

@media (min-width: 1024px) {

  .greetings h1,
  .greetings h3 {
    text-align: left;
  }
}
</style>
