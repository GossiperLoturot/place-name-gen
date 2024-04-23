import type { Component } from "solid-js";
import { For, Show, createEffect, createSignal } from "solid-js";
import init, { sample_once, sample_many } from "./pkg";
import { OutputRecord } from "./pkg/wasm";

const App: Component = () => {
  const [suspend, setSuspend] = createSignal(false);
  const [records, setRecords] = createSignal<OutputRecord[]>([]);

  createEffect(async () => {
    await init();
    setSuspend(true);
  }, []);

  const generate_once = async () => {
    setRecords([sample_once()]);
  };

  const generate_many = async (n: number) => {
    setRecords(sample_many(n));
  };

  return (
    <div class="container">
      <h1>place-name-gen</h1>
      <p>
        架空の地名を生成します。2つの英単語に分割できるアメリカの地名から前後の単語を収集し、ランダムにサンプルします。
      </p>
      <Show when={suspend()} fallback={<div>Loading...</div>}>
        <div class="btn-group">
          <button class="btn btn-default" onClick={generate_once}>
            Generate Once
          </button>
          <button class="btn btn-default" onClick={() => generate_many(10)}>
            Generate 10 times
          </button>
          <button class="btn btn-default" onClick={() => generate_many(100)}>
            Generate 100 times
          </button>
          <button class="btn btn-default" onClick={() => generate_many(1000)}>
            Generate 1,000 times
          </button>
        </div>
        <br></br>
        <ul>
          <For each={records()}>
            {(record) => (
              <li>
                <div>{record.en}</div>
                <div>{record.jp}</div>
              </li>
            )}
          </For>
        </ul>
      </Show>
    </div>
  );
};

export default App;
