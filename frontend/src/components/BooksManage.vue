<template>
  <v-app>
    <v-container fluid>
      <v-row>
        <v-col cols="3">
          <v-chip class="ma-2" v-for="btype in book_types" :key="btype" :color="book_type == btype ? 'red' : 'default'"
            @click="toggleBookType(btype)" label> {{ btype }} </v-chip>
        </v-col>

        <v-col cols="6">
          <v-chip class="ma-2" v-for="tag in tags" :key="tag" :color="used_tags.has(tag) ? 'green' : 'default'"
            @click="toggleTag(tag)" label> {{ tag }} </v-chip>
        </v-col>

        <v-col cols="3">
          <v-btn :disabled="selected_bookids.length == 0" variant="outlined" color="red"> 刪除 </v-btn>

          <v-menu>
            <template v-slot:activator="{ props }">
              <v-btn color="primary" v-bind="props" variant="outlined" :disabled="selected_bookids.length == 0">
                修改類別
              </v-btn>
            </template>
            <v-list>
              <v-list-item v-for="btype in book_types" :key="btype" :value="btype">
                <v-list-item-title>{{ btype }}</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-menu>
        </v-col>

      </v-row>
      <v-dialog transition="dialog-top-transition" width="auto" v-model="detail.show">
        <v-card>
          <v-card-title> {{ detail.book.title }}</v-card-title>
          <v-card-text>
            <v-row>
              <v-col cols="3"> 標題 </v-col>
              <v-col cols="9"> {{ detail.book.title }} </v-col>
              <v-col cols="3"> 類別 </v-col>
              <v-col cols="9"> {{ detail.book.book_type || "<NULL>" }} </v-col>
              <v-col cols="3"> 作者 </v-col>
              <v-col cols="9"> {{ detail.book.author }} </v-col>
              <v-col cols="3"> 標籤 </v-col>
              <v-col cols="9"> {{ detail.book.tags }} </v-col>
              <v-col cols="3"> 新增日期 </v-col>
              <v-col cols="9"> 2023/02/02 19:00:00 </v-col>
              <v-col cols="3"> 編輯日期 </v-col>
              <v-col cols="9"> 2023/02/02 19:00:00 </v-col>
              <v-col cols="3"> 網頁觀看位置 </v-col>
              <v-col cols="9"> <a href="/aaa">aaa</a> </v-col>
              <v-col cols="3"> 實際資料夾位置 </v-col>
              <v-col cols="9"> C:/aaa </v-col>
              <v-col cols="3"> 實際檔案 </v-col>
              <v-col cols="9"> [ "png", "txt" ] </v-col>
            </v-row>
          </v-card-text>
        </v-card>
      </v-dialog>
      <v-table>
        <thead>
          <tr>
            <th class="text-left">
              選擇
            </th>
            <th class="text-left">
              類別
            </th>
            <th class="text-left">
              標題
            </th>
            <th class="text-left">
              作者
            </th>
            <th class="text-left">
              標籤
            </th>
            <th class="text-left">
              詳細資料
            </th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="book in show_books" :key="book.book_id">
            <td class="text-xs-center">
              <v-checkbox class="d-inline-flex" :value="book.book_id" v-model="selected_bookids">
              </v-checkbox>
            </td>
            <td>{{ book.book_type || '<NULL>' }}</td>
            <td>{{ book.title }}</td>
            <td>{{ book.author }}</td>
            <td>
              <v-chip class="ma-2" label v-for="tag in book.tags" :key="tag"> {{ tag }} </v-chip>
            </td>
            <td>
              <v-btn v-on:click="show_book_detail(book.book_id)">詳細資料</v-btn>
            </td>
          </tr>
        </tbody>
      </v-table>
    </v-container>
  </v-app>
</template>

<script>
export default {
  data: () => {
    return {
      selected_bookids: [],
      books: [],
      book_types: [],
      book_type: null,

      tags: [],
      used_tags: new Set(),

      detail: {
        show: false,
        book_id: null,
        book: {},
      }
    }
  },
  created() {
    // fetch on init
    this.fetchData()
  },
  computed: {
    show_books() {
      return this.books.filter(book => {
        var match = true
        this.used_tags.forEach(tag => {
          if (!book.tags_set.has(tag)) {
            match = false
          }
        })

        if (this.book_type == null) {
          // ok
        } else if (this.book_type == '<NULL>' && book.book_type == undefined) {
          // ok
        } else if (this.book_type == book.book_type) {
          // ok
        } else {
          match = false
        }

        return match
      })
    }
  },
  methods: {
    async fetchData() {
      this.books = (await (await fetch('/api/books')).json()).books.map(book => {
        book.tags_set = new Set(book.tags || [])
        return book
      })
      this.tags = [...new Set(this.books.map(book => book.tags || []).flat())]
      this.book_types = [...new Set(this.books.map(book => book.book_type || '<NULL>'))]
    },
    toggleTag(tag) {
      if (this.used_tags.has(tag)) {
        this.used_tags.delete(tag)
      } else {
        this.used_tags.add(tag)
      }
    },
    toggleBookType(btype) {
      if (this.book_type == btype) {
        this.book_type = null
      } else {
        this.book_type = btype
      }
    },
    show_book_detail(book_id) {
      this.detail.show = true
      this.detail.book_id = book_id
      this.detail.book = this.books.find(book => book.book_id == book_id)
    }
  },
}
</script>
