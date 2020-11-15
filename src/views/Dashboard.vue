<template>
  <div class="dashboard-page">
    <div class="page-title">
      Dashboard
    </div>
    <div class="page-subtitle">Check your connection, aircraft profile, and manage clients</div>
    <div class="grid">
      <div class="custom-card">
        TEST
      </div>
      <div class="custom-card">
        TEST
      </div>
      <div class="custom-card">
        TEST
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Home',
  data:()=>({
  })
}
</script>

<style lang="scss" scoped>
.dashboard-page{

  .page-title{
    display: flex;
    align-items: center;
    height: 45px;
    justify-content: safe;
    font-weight: 500;
  }
  .page-subtitle{
    transform: translateY(-5px);
    font-weight: 400;
  }

  .grid{
    display:grid;
    background-color: #FFFFFF;
    grid-template-columns: auto auto auto;
    column-gap: 3rem;
    margin-top: 4rem;
  }

  .custom-card{
    border:2px solid #70707026;
    padding:21px 28px;
    border-radius:12px;
  }

}
</style>
