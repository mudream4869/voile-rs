<template>
  <v-app>
    <v-main>
      <v-container class="fluid fill-height">
        <v-layout class="align-center justify-center">
          <div class="v-col-xs-12 v-col-sm-8 v-col-md-4">
            <v-card class="elevation-4">
              <v-toolbar color="primary">
                <v-toolbar-title>Login form</v-toolbar-title>
              </v-toolbar>
              <v-card-text>
                <form ref="form" @submit.prevent="login()">
                  <v-text-field v-model="username" label="Username" type="text" required></v-text-field>

                  <v-text-field v-model="password" label="Password" type="password" required></v-text-field>
                  <v-btn type="submit" class="mt-4" color="primary" value="log in">Login</v-btn>
                </form>
              </v-card-text>
            </v-card>
          </div>
        </v-layout>
      </v-container>
    </v-main>
  </v-app>
</template>

<script>
export default {
  data: () => {
    return {
      username: '',
      password: '',
    }
  },
  created() {
  },
  methods: {
  },
}
</script>
