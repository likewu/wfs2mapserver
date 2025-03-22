import { rectSortingStrategy } from "@dnd-kit/sortable";
import { MultipleContainers } from "./MultipleContainers";

export default function Page() {
  return (
    <div className="App">
      <MultipleContainers
        itemCount={5}
        strategy={rectSortingStrategy}
        vertical
      />
    </div>
  );
}
