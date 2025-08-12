import { invoke } from '@tauri-apps/api/core';
import './App.css';

function App() {
	return (
		<main className='container'>
			<h1>NSPanel</h1>
			<button
				className='btn'
				onClick={() => {
					invoke('close_panel');
				}}>
				Close Panel
			</button>
		</main>
	);
}

export default App;
