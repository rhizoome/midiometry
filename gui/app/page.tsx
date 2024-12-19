'use client';

import React, { useEffect, useRef, useState } from 'react';

const NOTE_ON = 144;
const NOTE_OFF = 128;
const NOTE_NAMES = [
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
// https://colorkit.co/palette/00202e-003f5c-2c4875-8a508f-bc5090-ff6361-ff8531-ffa600-ffd380/
//  #ffadad, #ffd6a5, #fdffb6, #caffbf, #9bf6ff, #a0c4ff, #bdb2ff and #ffc6ff.
const COLORS = [
	'255,173,173',
	'255,214,165',
	'253,255,182',
	'202,255,191',
	'155,246,255',
	'160,196,255',
	'189,178,255',
	'255,198,255',
];

export default function Home() {
	const [notes, setNotes] = useState<number[]>([]);

	useEffect(() => {
		window.onPluginMessage = (message: number[]) => {
			const noteStatus = message[0];

			// TODO:
			// make use of velocity!
			//const velocity = message[2];
			const noteNumber = message[1];

			if (noteStatus === NOTE_ON) {
				console.log('NOTE ON:', message);

				setNotes((prevState) => [...prevState, noteNumber]);
			} else if (noteStatus === NOTE_OFF) {
				console.log('NOTE OFF:', message);
				// 0 velocity = note OFF
				setNotes((prevState) =>
					prevState.filter((note) => note !== noteNumber)
				);
			}
		};
	}, []);

	return (
		<div className='overflow-hidden h-screen w-screen bg-[#121212] text-white'>
			{/* --- background title text thingy --- */}
			<div className='flex h-full w-full justify-center items-center opacity-25 text-4xl'>
				<h1>MIDIOMETRY</h1>
			</div>

			{/* --- credits! --- */}
			<div className='absolute bottom-0 right-0 opacity-25 m-1'>
				<p>a plugin by dvub</p>
			</div>

			<Dodecagon notes={notes} />
		</div>
	);
}

const Dodecagon = (props: { notes: number[] }) => {
	const { notes } = props;

	const canvasRef = useRef<HTMLCanvasElement | null>(null);

	useEffect(() => {
		// --- init setup --- //
		const refCurrent = canvasRef.current!;
		const ctx = refCurrent.getContext('2d')!;

		ctx.clearRect(0, 0, refCurrent.width, refCurrent.height);
		ctx.textAlign = 'center';

		const centerX = refCurrent.width / 2;
		const centerY = refCurrent.height / 2;
		const radius = Math.min(refCurrent.width, refCurrent.height) * 0.33;

		// this has to be here because we need canvas context for center coords and radius
		const coordinates = generateCoordinates(
			{ x: centerX, y: centerY },
			radius
		);

		// --- add note names in slightly larger circle --- //
		const textOffset = 25;
		generateCoordinates(
			{ x: centerX, y: centerY },
			radius + textOffset
		).forEach((coordinate, index) => {
			const text = NOTE_NAMES[index];
			ctx.fillStyle = 'white';
			ctx.fillText(text, coordinate.x, coordinate.y);
		});

		// --- draw dots to represent when notes are played --- //
		notes.map((note) => {
			const octave = Math.floor(note / 12);

			const resultCoordinates = findNoteCoordinates(note, coordinates);
			const radius = octave / 10;

			ctx.strokeStyle = `rgba(255,255,255,0.25)`;
			ctx.lineWidth = 1;

			ctx.beginPath();
			ctx.arc(
				resultCoordinates.x,
				resultCoordinates.y,
				25 * radius,
				0,
				Math.PI * 2
			);
			ctx.stroke();
		});

		// --- CONNECTING LINES --- //
		const lineOpacity = 0.5;
		const lineWidth = 2;

		for (let i = 0; i < notes.length - 1; i++) {
			// this is a stupidly overcomplicated wawy to do shit
			const results = Array(2)
				.fill(0)
				.map((_, j) => {
					const note = notes[i + j];
					return findNoteCoordinates(note, coordinates);
				});

			ctx.lineWidth = lineWidth;

			const color =
				COLORS[Math.abs(notes[i] - notes[i + 1]) % COLORS.length];
			ctx.strokeStyle = `rgba(${color},${lineOpacity}`;

			ctx.beginPath();

			ctx.moveTo(results[0].x, results[0].y);
			ctx.lineTo(results[1].x, results[1].y);

			ctx.stroke();
		}
	}, [notes]);

	return (
		<div className='absolute top-0 left-0'>
			<canvas ref={canvasRef} width='400' height='400' />
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

function findNoteCoordinates(
	note: number,
	coordinates: Array<{ x: number; y: number }>
) {
	const normalizedNote = note % 12;
	return coordinates.find((_, i) => (i * 7) % 12 === normalizedNote)!;
}
