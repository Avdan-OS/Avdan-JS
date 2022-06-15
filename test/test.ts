import { Task } from "./index";

const a: Task<Task.Event.Type.Progressable & Task.Event.Type.Milestone<["downloaded", "uploaded"]>>;
