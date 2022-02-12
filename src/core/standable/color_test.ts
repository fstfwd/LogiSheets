import {StandardColor} from './color'
describe('standard color test', () => {
    it('argb convert', () => {
        const c = StandardColor.fromArgb('FF4178B8')
        expect(c.css()).toBe('rgba(65, 120, 184, 1)')
    })
})
