'use client';

import React, { useEffect, useRef, useState } from 'react';

export default function Home() {
	const [notes, setNotes] = useState<number[]>([]);

	useEffect(() => {
		
		window.onPluginMessage = (message: number[]) => {
			const velocity = message[2];
			const noteNumber = message[1];

			if (velocity > 0) {
				console.log('NOTE ON:', message);

				setNotes((prevState) => [...prevState, noteNumber]);
			} else {
				console.log('NOTE OFF:', message);
				// 0 velocity = note OFF
				setNotes((prevState) =>
					prevState.filter((note) => note !== noteNumber)
				);
			}

			//const event = new CustomEvent('pluginMessage', { detail: message });
			// window.dispatchEvent(event);
		};
	}, []);

	return (
		<div className=' overflow-hidden'>
			<Dodecagon notes={notes} />
		</div>
	);
}

const Dodecagon = (props: { notes: number[] }) => {
	const { notes } = props;

	const canvasRef = useRef<HTMLCanvasElement | null>(null);

	useEffect(() => {}, []);

	useEffect(() => {
		// --- init setup --- //

		const refCurrent = canvasRef.current!;
		const ctx = refCurrent.getContext('2d')!;

		ctx.clearRect(0, 0, refCurrent.width, refCurrent.height);

		const centerX = refCurrent.width / 2;
		const centerY = refCurrent.height / 2;
		const radius = Math.min(refCurrent.width, refCurrent.height) * 0.33;

		const coordinates = generateCoordinates(
			{ x: centerX, y: centerY },
			radius
		);
		const noteNames = [
			'C',
			'G',
			'D',
			'A',
			'E',
			'B',
			'F#/Gb',
			'C#/Db',
			'Ab',
			'Eb',
			'Bb',
			'F',
		];

		ctx.lineWidth = 2;

		ctx.strokeStyle = `rgba(0,0,0,1)`;
		ctx.arc(centerX, centerY, radius, 0, Math.PI * 2);
		// ctx.stroke();

		coordinates.forEach((coordinate, index) => {
			const text = noteNames[index];
			ctx.fillText(text, coordinate.x + 10, coordinate.y);
		});

		for (let i = 0; i < notes.length - 1; i++) {
			const note = notes[i];
			const normalizedNote = note % 12;

			const result = coordinates.find(
				(x, i) => (i * 7) % 12 === normalizedNote
			)!;

			const nextNote = notes[i + 1];
			const nextIndex = nextNote % 12;

			const result2 = coordinates.find(
				(x, i) => (i * 7) % 12 === nextIndex
			)!;

			ctx.beginPath();
			ctx.strokeStyle = `rgba(0,0,0,${0.25}`;

			ctx.moveTo(result.x, result.y);
			ctx.lineTo(result2.x, result2.y);

			ctx.stroke();
		}
	}, [notes]);

	return (
		<div>
			<canvas
				ref={canvasRef}
				width='400'
				height='400'
				style={{ border: '1px solid black' }}
			/>
		</div>
	);
};

function generateCoordinates(
	center: { x: number; y: number },
	radius: number
): Array<{ x: number; y: number }> {
	const coordinates = [];
	for (let i = 0; i < 12; i++) {
		const angle = (i * Math.PI) / 6;
		const x = center.x + radius * Math.cos(angle);
		const y = center.y + radius * Math.sin(angle);
		coordinates.push({ x, y });
	}
	return coordinates;
}
