<template>
  <v-app>
    <v-form class="ma-md-2">
      <v-file-input accept="text/*, application/zip, application/pdf" multiple label="txt檔案/zip檔案/pdf檔案"
        :key="fileInputKey" @change="addBook($event)"></v-file-input>
    </v-form>
    <v-btn @click="uploadBooks()" :disabled="uploadable == 0"> 上傳 </v-btn>
    <div class="ma-md-2">
      <v-list>
        <v-list-item v-for="item in readyFiles" :key="item.filename" :title="item.filename">
          <v-alert color="success" text="上傳成功" v-if="item.status == 1"></v-alert>
          <v-alert color="warning" :text="'上傳失敗: ' + item.error_log" v-if="item.status == 2"></v-alert>
        </v-list-item>
      </v-list>

      <h2> 可上傳檔案格式: txt/zip/pdf </h2>
    </div>
  </v-app>
</template>

<script>
export default {
  data: () => {
    return {
      fileInputKey: 0,
      alertFail: false,
      alertSuccess: false,

      readyFiles: [{
        filename: '',
        file: null,
        status: 0, // 0: not upload, 1: success, 2: failure
        error_log: '',
      }],
    }
  },
  created() {
    this.readyFiles = [];
  },
  computed: {
    uploadable() {
      var cnt = 0
      for (var readyFile of this.readyFiles) {
        if (readyFile.status != 1) {
          cnt += 1
        }
      }
      return cnt
    }
  },
  methods: {
    addBook(event) {
      if (!event.target.files) {
        return
      }

      for (var book_file of event.target.files) {
        this.readyFiles.push({
          filename: book_file.name,
          file: book_file,
          status: 0,
          error_log: '',
        })
      }

      this.fileInputKey++
    },

    async uploadBooks() {
      if (this.uploadable == 0) {
        return
      }

      for (var readyFile of this.readyFiles) {
        if (readyFile.status == 1) {
          continue
        }

        const formData = new FormData()
        formData.append('book', readyFile.file, readyFile.filename)

        let filename = readyFile.filename

        fetch(`api/books`, {
          method: 'POST',
          body: formData,
        }).then(async res => {
          var item = this.readyFiles.find(item => item.filename == filename)
          if (res.status == 200) {
            item.status = 1
          } else {
            item.status = 2
            item.error_log = await res.text();
          }
        })
      }
    },
  }
}
</script>
