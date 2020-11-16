<template>
  <div class="dashboard-page">
    <div class="page-title">
      Dashboard
    </div>
    <div class="page-subtitle">Check your connection, aircraft profile, and manage clients</div>
    <div class="grid">
      <div class="custom-card">
        <div class="custom-title">
          Sim Connect
        </div>
        <div class="status">
          <div class="status-indicator"></div>
          Not Connected
        </div>
      </div>
      <div class="custom-card">
        <div class="custom-title">
          Client Conection
        </div>
        <div class="status">
          <div class="status-indicator"></div>
          Not Connected
        </div>
      </div>
      <div class="custom-card aircraft">
        <div class="custom-title">
          Selected Aircraft
          <v-menu offset-y bottom left>
            <template v-slot:activator="{ on, attrs }">
              <v-btn color="primary" class="menu-dropdown" text v-bind="attrs" v-on="on">
              </v-btn>
            </template>
            <v-list>
              <v-list-item link>
                <v-list-item-title>A32NX</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-menu>
        </div>
        <div class="aircraft-info">
          <div class="pre-view"></div>
          <div class="aircraft-name">A32NX</div>
        </div>
      </div>
      <div class="custom-card connection-status">
        <div class="custom-title">
          Connection Status
        </div>
        <div class="custom-card">
          <div class="custom-title">
            Ping
          </div>
        </div>
        <div class="custom-card">
          <div class="custom-title">
            Packet Loss
          </div>
        </div>
        <div class="custom-card">
          <div class="custom-title">
            Download Rate
          </div>
        </div>
        <div class="custom-card">
          <div class="custom-title">
            Upload Rate
          </div>
        </div>
      </div>
      <div class="custom-card client-list">
        <div class="custom-title">
          Connected Pilots
        </div>
        Here you'll be able to see all connected pilots. 
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
    font-weight: 600;
    font-size: 16px;
  }
  .page-subtitle{
    transform: translateY(-5px);
    font-weight: 500;
    color: #9CAAC1;
  }


  .custom-card{
    border: 2px solid rgba(112, 112, 112, 0.149);
    padding: 15px 20px;
    border-radius: 12px;
    .custom-title{
      font-weight: 600;
      font-size: 14px;
      height: 21px;
    }
    .status{
      width: 100%;
      height: 100%;
      display:flex;
      align-items: center;
      .status-indicator{
        width: 30px;
        height: 30px;
        background-color: #F85560;
        border-radius: 50%;
        margin-right: 10px;;
      }
    }
  }
  .custom-card.aircraft{
    background-color: #259efb;
    color:#FFFFFF;
    .aircraft-info{
      display:flex;
      justify-content: space-between;

      .pre-view{
        background-color:#FFFFFF;
        margin-top: 15px;
        height: 55px;
        width: 65%;
        border-radius: 9px;
        background-position: center;
        background-size: 65%;
        background-image: url("../assets/icons/plane.png");
      }
      .aircraft-name{
        align-self: center;
        width: 15%;
      }
    }
    .custom-title{
      .menu-dropdown{
        background-color: #FFFFFF;
        min-width: 25px !important;
        max-height: 25px;
        float: right;
        background-position: center;
        background-size: 65%;
        background-image: url("../assets/icons/arrow-down-0375F9.png");
      }
    }
  }
  .custom-card.connection-status{
    background-color: #259efb;
    grid-column-start: 1;
    grid-column-end: 3;
    display: grid;
    grid-template-columns: auto auto;
    grid-template-rows: 21px auto auto;
    column-gap: 1rem;
    row-gap: 1rem;
    .custom-title{
      color: #FFFFFF;
      grid-column-start: 1;
      grid-column-end: 3;
    }
    .custom-card{
      background-color: rgba(255, 255, 255, 0.151);
    }
  }
  .custom-card.client-list{
    grid-column-start: 3;
    grid-column-end: 4;
    height: 340px;
  }


  .grid{
    display:grid;
    background-color: #FFFFFF;
    grid-template-columns: 15.2rem 15.2rem 15.2rem;
    grid-template-rows: auto auto;
    column-gap: 2.2rem;
    row-gap: 2rem;
    margin-top: calc(2rem + 15px);
  }
}
</style>
