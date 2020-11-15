<template>
  <v-app>
    <div class="grid">
        <ul>
          <li>
            <div class="user-info"><div class="user-img"></div>SafeShows</div>
            <div class="hashtag">#0001</div>
          </li>
          <li><router-link to="/"> <div class="icon-dashboard" /> Dashboard</router-link></li>
          <li><router-link to="connect"> <div class="icon-dashboard" /> Connect</router-link></li>
          <li><router-link to="server"> <div class="icon-server" /> Server</router-link></li>
          <li>
            <div>SETTINGS</div>
          </li>
          <li><router-link to="settings"> <div class="icon-settings" /> Settings</router-link></li>
          <li><router-link to="discord-ettings"> <div class="icon-headphones" /> Discord Settings</router-link></li>
          <li class="bottom-nav-text">
            <div>YourControls. 2020</div>
            <div class="version">v3.0.0</div>
          </li>
        </ul>
        <div class="pageView">
          <router-view />
        </div>
    </div>
  </v-app>
</template>

<script>
import { promisified } from 'tauri/api/tauri'

export default {
  name: 'App',

  data: () => ({
    //
  }),


  created:() => {
    console.log("SEND")
    var a = {
      cmd: "startServerCommand",
      payload: {
        port: 9999,
        ip: "10.2.5.10",
        userName: "Hello"
      }
    };

    promisified(a).then((success) => {
      console.log({success})
    }).catch((error) => {
      console.error({error})
    });
  }

};
</script>

<style lang="scss">
  .pageView{
    padding: 2rem 75px;
    border-radius: 40px 0 0 40px;
    background-color: #FFFFFF !important;
    width: auto;
    box-shadow: 0 0 50px #00000020;
  }

  body{
    background-color: #EEF4FD !important;
  }

  .row{
    background-color: #EEF4FD !important;
  }

  ul {
    list-style-type: none;
    margin: 0;
    padding: 2rem 1rem;
    height: 100vh;
    background-color: #EEF4FD;
  }

  li{
    div{
      margin-top: 4rem;
      margin-bottom: 7px;
      padding: 0 16px;
    }
    .user-info{
      margin-top: 0;
      margin-bottom: 5rem;
      padding: 0 16px;
      display: flex;
      color: #0E2B5E !important;
      text-decoration: none;
      align-items: center;
      font-weight: 600;
      font-size: 18px;
      transform: translateY(-5px);
      .user-img{
        border-radius:50%;
        background-color:#0075FF;
        height:45px;
        width:45px;
        margin:0;
        margin-right: 16px;
        padding:0;
        transform: translateY(9px);
      }
    }
    .hashtag{
      padding: 0 16px;
      margin:0;
      transform: translate( 60px, calc( -5rem + -18px) );
      font-weight: 300;
    }

    
  }

  .bottom-nav-text{
    div{
      margin: 0;
      margin-top: calc(100% + 15px);
      font-weight: 400;
    }
    .version{
      margin: 0;
      font-weight: 200;
    }
  }

  li a {
    display: flex;
    color: #9CAAC1 !important;
    height: 60px;
    padding: 0 16px;
    text-decoration: none;
    align-items: center;
    div {
      margin: 0;
      margin-right: 10px;
      padding: 0;
      height: 24px;
      width: 24px;
    }
  }

  .router-link-exact-active {
    border-radius: 15px;
    box-shadow: 0 3px 6px #00000020;
    background-color: #0375F9;
    color: white !important;
    line-height: 1.5;
    vertical-align: middle;
    .icon-dashboard{
      background-image: url("./assets/icons/dashboard-white.png");
    }
    .icon-server{
      background-image: url("./assets/icons/server-white.png");
    }
    .icon-settings{
      background-image: url("./assets/icons/settings-white.png");
    }
    .icon-headphones{
      background-image: url("./assets/icons/headphones-white.png");
    }
  }

  .grid{
    display: grid;
    grid-template-columns: 20rem auto;
    background-color:#EEF4FD;
  }

  .icon-dashboard{
    background-size: 100%;
    background-image: url("./assets/icons/dashboard-9CAAC1.png");
  }
  .icon-server{
    background-size: 100%;
    background-image: url("./assets/icons/server-9CAAC1.png");
  }
  .icon-settings{
    background-size: 100%;
    background-image: url("./assets/icons/settings-9CAAC1.png");
  }
  .icon-headphones{
    background-size: 100%;
    background-image: url("./assets/icons/headphones-9CAAC1.png");
  }

  $body-font-family: "Montserrat";
  $title-font: "Quicksand";

  .v-application {
    font-family: $body-font-family, sans-serif !important;
    font-size: 1rem !important;
    a {
      font-family: $body-font-family, sans-serif !important;
      font-weight: 600 !important;
    }
    .display-1 {
      // To pin point specific classes of some components
      font-family: $title-font, sans-serif !important;
    }
    .display-2 {
      // To pin point specific classes of some components
      font-family: $title-font, sans-serif !important;
    }
    .display-3 {
      // To pin point specific classes of some components
      font-family: $title-font, sans-serif !important;
    }
    .display-4 {
      // To pin point specific classes of some components
      font-family: $title-font, sans-serif !important;
    }
    .display-5 {
      // To pin point specific classes of some components
      font-family: $title-font, sans-serif !important;
    }
    .headline {
      // To pin point specific classes of some components
      font-family: $title-font, sans-serif !important;
    }
    .title {
      // To pin point specific classes of some components
      font-family: $title-font, sans-serif !important;
    }
    .v-list-item__title {
      // To pin point specific classes of some components
      font-family: $title-font, sans-serif !important;
    }
    .v-card__title {
      // To pin point specific classes of some components
      font-family: $title-font, sans-serif !important;
    }
  }
</style>