<script>
	// Add any logic here if needed
</script>

<style>
	.hero-section {
		min-height: 100vh;
		background: #ffb3ba;
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		overflow: hidden;
	}

	.campus-image-frame {
		position: fixed;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 60vw;
		max-width: 800px;
		height: 50vh;
		clip-path: polygon(0% 0%, 100% 0%, 100% 70%, 90% 100%, 10% 100%, 0% 70%);
		border: 4px solid #000;
		box-shadow: 12px 12px 0 #000;
		z-index: 0;
	}

	.campus-image-frame img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		object-position: center;
	}

	.content-wrapper {
		position: relative;
		z-index: 1;
		max-width: 600px;
		padding: 2rem;
		margin-top: 60vh;
	}

	.protocol-title {
		font-size: 3rem;
		font-weight: 900;
		color: #000;
		text-align: center;
		margin-bottom: 2rem;
		text-shadow: 3px 3px 0 #fff;
	}

	.protocol-text {
		font-size: 1.1rem;
		font-weight: 600;
		color: #000;
		text-align: center;
		margin-bottom: 2rem;
		background: rgba(255, 255, 255, 0.8);
		padding: 1.5rem;
		border: 3px solid #000;
		box-shadow: 6px 6px 0 #000;
	}

	.cosmic-portal-btn {
		--btn-bg: #1a1a2e;
		--btn-color: #e94560;
		--btn-highlight: #16213e;
		--portal-color1: #0f3460;
		--portal-color2: #e94560;

		position: relative;
		font-family: Arial, sans-serif;
		font-size: 1.2em;
		font-weight: bold;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--btn-color);
		background-color: var(--btn-bg);
		border: none;
		padding: 1em 2em;
		cursor: pointer;
		overflow: hidden;
		transition: all 0.3s ease;
		clip-path: polygon(0% 0%, 100% 0%, 100% 70%, 90% 100%, 10% 100%, 0% 70%);
		display: block;
		margin: 0 auto;
	}

	.cosmic-portal-btn::before {
		content: "";
		position: absolute;
		top: -50%;
		left: -50%;
		width: 200%;
		height: 200%;
		background: conic-gradient(
			from 0deg at 50% 50%,
			var(--portal-color1) 0deg,
			var(--portal-color2) 120deg,
			var(--portal-color1) 240deg,
			var(--portal-color2) 360deg
		);
		animation: rotatePortal 10s linear infinite;
		opacity: 0;
		transition: opacity 0.3s ease;
	}

	.cosmic-portal-btn:hover::before {
		opacity: 0.8;
	}

	.cosmic-portal-btn::after {
		content: "";
		position: absolute;
		inset: 0.15em;
		background-color: var(--btn-bg);
		clip-path: inherit;
		z-index: 1;
	}

	.btn-text {
		position: relative;
		z-index: 2;
	}

	.portal-effect {
		position: absolute;
		inset: 0;
		background: radial-gradient(
			circle at 50% 50%,
			var(--btn-highlight),
			transparent 50%
		);
		opacity: 0;
		transition: opacity 0.3s ease;
		z-index: 1;
	}

	.cosmic-portal-btn:hover .portal-effect {
		opacity: 1;
		animation: pulsePortal 2s ease-in-out infinite;
	}

	.cosmic-portal-btn:hover {
		transform: translateY(-0.25em);
		box-shadow: 0 0.5em 1em rgba(233, 69, 96, 0.2);
	}

	.cosmic-portal-btn:active {
		transform: translateY(0);
	}

	.cosmic-portal-btn:focus {
		outline: none;
		box-shadow: 0 0 0 0.2em rgba(233, 69, 96, 0.5);
	}

	@keyframes rotatePortal {
		0% {
			transform: rotate(0deg);
		}
		100% {
			transform: rotate(360deg);
		}
	}

	@keyframes pulsePortal {
		0%,
		100% {
			transform: scale(1);
		}
		50% {
			transform: scale(1.05);
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.cosmic-portal-btn::before,
		.cosmic-portal-btn:hover .portal-effect {
			animation: none;
		}
	}
</style>

<div class="hero-section">
	<div class="campus-image-frame">
		<img src="/1000111266.webp" alt="Campus" />
	</div>

	<div class="content-wrapper">
		<h1 class="protocol-title">THE AEGIS PROTOCOL</h1>
		<p class="protocol-text">
			For years, the Great Institute has functioned in fragmented
			silence. Knowledge is scattered across unofficial drives,
			grievances vanish into administrative voids, and students wander
			lost between hostel blocks and academic buildings. Scholars pool
			their resources for transport in secret WhatsApp groups,
			research opportunities reach only the chosen few, and lost
			belongings drift unclaimed in the shadows of campus corners.
		</p>
		<button class="cosmic-portal-btn">
			<span class="btn-text">The chaos must end.</span>
			<div class="portal-effect"></div>
		</button>
	</div>
</div>
