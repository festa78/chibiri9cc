package main

import (
	"fmt"
	"io/ioutil"
	"net/http"
)

func handler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "Hello, World in go")

	resp, err := http.Get("nginx-srv/compilerapi")
	if err != nil {
		_ = fmt.Errorf("fail on compilerapi: %w", err)
		return
	}

	defer resp.Body.Close()
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		_ = fmt.Errorf("fail on compilerapi: %w", err)
		return
	}

	fmt.Println(string(body))
}

func main() {
	http.HandleFunc("/", handler) // ハンドラを登録してウェブページを表示させる
	http.ListenAndServe(":8080", nil)
}
