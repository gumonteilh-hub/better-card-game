
import { describe, it, expect } from 'vitest';
import { attackReady } from './gameRules';
import { ICardInstance } from '../types/game';

describe('attackReady', () => {
    it('should return true for Windfury monster that has attacked once', () => {
        const card: ICardInstance = {
            id: 1,
            template_id: 1,
            name: "Windfury Monster",
            description: "",
            race: "COMMON",
            class: "COMMON",
            cost: 1,
            owner: "player1",
            location: { type: "Field", value: 0 },
            cardType: {
                type: "monster",
                attack: 1,
                hp: 1,
                max_hp: 1,
                asleep: false,
                attackCount: 1, // Already attacked once
                keywords: ["Windfury"],
                onPlay: [],
                onAttack: [],
                onDeath: []
            },
            playTarget: undefined
        };

        expect(attackReady(card)).toBe(true);
    });

    it('should return false for Windfury monster that has attacked twice', () => {
        const card: ICardInstance = {
            id: 1,
            template_id: 1,
            name: "Windfury Monster",
            description: "",
            race: "COMMON",
            class: "COMMON",
            cost: 1,
            owner: "player1",
            location: { type: "Field", value: 0 },
            cardType: {
                type: "monster",
                attack: 1,
                hp: 1,
                max_hp: 1,
                asleep: false,
                attackCount: 2,
                keywords: ["Windfury"],
                onPlay: [],
                onAttack: [],
                onDeath: []
            },
            playTarget: undefined
        };

        expect(attackReady(card)).toBe(false);
    });

    it('should return false for normal monster that has attacked once', () => {
        const card: ICardInstance = {
            id: 1,
            template_id: 1,
            name: "Normal Monster",
            description: "",
            race: "COMMON",
            class: "COMMON",
            cost: 1,
            owner: "player1",
            location: { type: "Field", value: 0 },
            cardType: {
                type: "monster",
                attack: 1,
                hp: 1,
                max_hp: 1,
                asleep: false,
                attackCount: 1,
                keywords: [],
                onPlay: [],
                onAttack: [],
                onDeath: []
            },
            playTarget: undefined
        };

        expect(attackReady(card)).toBe(false);
    });
});
