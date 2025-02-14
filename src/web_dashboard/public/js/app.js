class ASAMDashboard {
	constructor() {
		this.safeStatusEl = document.getElementById('safe-status');
		this.defiStrategyEl = document.getElementById('defi-strategy');
		this.init();
	}

	async init() {
		await this.updateSafeStatus();
		await this.updateDefiStrategy();
		setInterval(() => this.updateAll(), 30000);
	}

	async updateAll() {
		await Promise.all([
			this.updateSafeStatus(),
			this.updateDefiStrategy()
		]);
	}

	async updateSafeStatus() {
		try {
			const response = await fetch('/api/safe/status');
			const data = await response.json();
			this.safeStatusEl.innerHTML = this.renderSafeStatus(data);
		} catch (error) {
			console.error('Failed to update safe status:', error);
			this.safeStatusEl.innerHTML = 'Error loading safe status';
		}
	}

	async updateDefiStrategy() {
		try {
			const response = await fetch('/api/defi/strategy');
			const data = await response.json();
			this.defiStrategyEl.innerHTML = this.renderDefiStrategy(data);
		} catch (error) {
			console.error('Failed to update DeFi strategy:', error);
			this.defiStrategyEl.innerHTML = 'Error loading DeFi strategy';
		}
	}

	renderSafeStatus(data) {
		return `
			<div class="space-y-2">
				<p class="text-sm text-gray-600">Balance: ${data.balance} ETH</p>
				<p class="text-sm text-gray-600">Last Transaction: ${data.lastTx}</p>
			</div>
		`;
	}

	renderDefiStrategy(data) {
		return `
			<div class="space-y-2">
				<p class="text-sm text-gray-600">Current APY: ${data.apy}%</p>
				<p class="text-sm text-gray-600">Strategy: ${data.strategy}</p>
			</div>
		`;
	}
}

document.addEventListener('DOMContentLoaded', () => {
	new ASAMDashboard();
});