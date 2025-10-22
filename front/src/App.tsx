import "./css/App.css";
import "./css/Hud.css";
import "./css/Card.css";
import "./css/Board.css";
import "./css/Home.css";
import "./css/Collection.css";
import { createRouter, RouterProvider } from "@tanstack/react-router";
import { routeTree } from "./routeTree.gen";
import { Toaster } from "sonner";

const router = createRouter({ routeTree });

function App() {
	return (
		<>
			<Toaster position="top-center" richColors />
			<RouterProvider router={router}></RouterProvider>
		</>
	);
}

export default App;
