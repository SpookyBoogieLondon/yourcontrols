import Vue from 'vue';
import Vuetify from 'vuetify/lib';

Vue.use(Vuetify);

export default new Vuetify({
  theme: {
      options: {
        customProperties: true,
      },
    themes: {
      light: {
        primary: '#FFFFFF',
        secondary: '#424242',
        accent: '#82B1FF',
        error: '#f85560',
        info: '#2196F3',
        success: '#00db9d',
        warning: '#FFC107'
      },
    },
  },
});
